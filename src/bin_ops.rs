// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;

use traits::{MatrixShape, MatrixRawGet, SameShape};
use row_accessor::RowAccessor;
use column_accessor::ColumnAccessor;
use transpose::Transposer;
use view::View;
use matrix_mul::MatrixMul;
use matrix::{Matrix, write_mat};
use un_ops::{MatrixUnOp, UnOp};

pub trait BinOp
{
	fn op(&self, a: f64, b: f64) -> f64;
}

macro_rules! op
{
	($name: ident, $op: tt) =>
	{
		#[deriving(Clone)]
		pub struct $name;
		impl $name
		{
			#[inline(always)]
			pub fn new() -> $name
			{
				$name
			}
		}
		impl BinOp for $name
		{
			#[inline(always)]
			fn op(&self, a: f64, b: f64) -> f64
			{
				expr!(a $op b)
			}
		}
	}
}

macro_rules! expr
{
	($e: expr) => { $e }
}

op!(OpAdd, +)
op!(OpSub, -)
op!(OpDiv, /)
op!(OpMul, *)

pub struct MatrixBinOp<TA, TB, TO>
{
	a: TA,
	b: TB,
	o: TO
}

impl<TA: MatrixShape,
     TB: SameShape,
     TO: BinOp>
MatrixBinOp<TA, TB, TO>
{
	pub unsafe fn unsafe_new(a: TA, b: TB, o: TO) -> MatrixBinOp<TA, TB, TO>
	{
		MatrixBinOp{ a: a, b: b, o: o}
	}

	pub fn new(a: TA, b: TB, o: TO) -> MatrixBinOp<TA, TB, TO>
	{
		assert!(b.same_shape(a.nrow(), a.ncol()));
		MatrixBinOp{ a: a, b: b, o: o }
	}
}

impl<TA: Clone,
     TB: Clone,
     TO: Clone>
Clone for
MatrixBinOp<TA, TB, TO>
{
	fn clone(&self) -> MatrixBinOp<TA, TB, TO>
	{
		MatrixBinOp{ a: self.a.clone(), b: self.b.clone(), o: self.o.clone() }
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TB: MatrixRawGet + SameShape,
     TO: BinOp>
MatrixRawGet for
MatrixBinOp<TA, TB, TO>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.o.op(self.a.raw_get(r, c), self.b.raw_get(r, c))
	}
}

impl<TA: MatrixShape,
	 TB,
     TO: BinOp>
MatrixShape for
MatrixBinOp<TA, TB, TO>
{
	fn nrow(&self) -> uint
	{
		self.a.nrow()
	}

	fn ncol(&self) -> uint
	{
		self.a.ncol()
	}
}

impl<TA: Container,
	 TB,
     TO: BinOp>
Container for
MatrixBinOp<TA, TB, TO>
{
	fn len(&self) -> uint
	{
		self.a.len()
	}
}

impl<TA: MatrixShape,
	 TB,
     TO: BinOp>
SameShape for
MatrixBinOp<TA, TB, TO>
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TB: MatrixRawGet + SameShape,
     TO: BinOp>
fmt::Show for
MatrixBinOp<TA, TB, TO>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
	}
}

macro_rules! bin_op
{
	($op_name: ident, $op_method: ident, $op: ident) =>
	{
		impl<RHS: MatrixRawGet + Clone + SameShape,
             TA: MatrixRawGet + Clone + MatrixShape,
             TB: MatrixRawGet + Clone + SameShape,
             TO: BinOp + Clone>
		$op_name<RHS, MatrixBinOp<MatrixBinOp<TA, TB, TO>, RHS, $op>> for
		MatrixBinOp<TA, TB, TO>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<MatrixBinOp<TA, TB, TO>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<RHS: MatrixRawGet + Clone + SameShape,
             TA: MatrixRawGet + Clone + MatrixShape,
             TO: UnOp + Clone>
		$op_name<RHS, MatrixBinOp<MatrixUnOp<TA, TO>, RHS, $op>> for
		MatrixUnOp<TA, TO>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<MatrixUnOp<TA, TO>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<'l,
		     RHS: MatrixRawGet + Clone + SameShape>
		$op_name<RHS, MatrixBinOp<&'l Matrix, RHS, $op>> for
		&'l Matrix
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<&'l Matrix, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS, MatrixBinOp<View<T>, RHS, $op>> for
		View<T>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<View<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS, MatrixBinOp<Transposer<T>, RHS, $op>> for
		Transposer<T>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<Transposer<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS, MatrixBinOp<RowAccessor<T>, RHS, $op>> for
		RowAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<RowAccessor<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS, MatrixBinOp<ColumnAccessor<T>, RHS, $op>> for
		ColumnAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<ColumnAccessor<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T1:   MatrixShape + Clone,
		     T2:   MatrixShape + Clone>
		$op_name<RHS, MatrixBinOp<MatrixMul<T1, T2>, RHS, $op>> for
		MatrixMul<T1, T2>
		{
			fn $op_method(&self, rhs: &RHS) -> MatrixBinOp<MatrixMul<T1, T2>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
	}
}

bin_op!(Add, add, OpAdd)
bin_op!(Sub, sub, OpSub)
bin_op!(Mul, mul, OpMul)
bin_op!(Div, div, OpDiv)
