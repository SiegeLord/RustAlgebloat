// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::ops::*;

use traits::{MatrixShape, MatrixRawGet, SameShape};
use row_accessor::RowAccessor;
use column_accessor::ColumnAccessor;
use transpose::Transposer;
use view::View;
use slice::Slice;
use reshape::Reshape;
use hstack::HStack;
use vstack::VStack;
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
		#[derive(Copy, Clone)]
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

op!(OpAdd, +);
op!(OpSub, -);
op!(OpDiv, /);
op!(OpMul, *);

#[derive(Copy)]
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
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
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
	fn nrow(&self) -> usize
	{
		self.a.nrow()
	}

	fn ncol(&self) -> usize
	{
		self.a.ncol()
	}
}

impl<TA: MatrixShape,
	 TB,
     TO: BinOp>
SameShape for
MatrixBinOp<TA, TB, TO>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TB: MatrixRawGet + SameShape,
     TO: BinOp>
fmt::Display for
MatrixBinOp<TA, TB, TO>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
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
		$op_name<RHS> for
		MatrixBinOp<TA, TB, TO>
		{
			type Output = MatrixBinOp<MatrixBinOp<TA, TB, TO>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<MatrixBinOp<TA, TB, TO>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<RHS: MatrixRawGet + Clone + SameShape,
             TA: MatrixRawGet + Clone + MatrixShape,
             TO: UnOp + Clone>
		$op_name<RHS> for
		MatrixUnOp<TA, TO>
		{
			type Output = MatrixBinOp<MatrixUnOp<TA, TO>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<MatrixUnOp<TA, TO>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<'l,
		     RHS: MatrixRawGet + Clone + SameShape>
		$op_name<RHS> for
		&'l Matrix
		{
			type Output = MatrixBinOp<&'l Matrix, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<&'l Matrix, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		View<T>
		{
			type Output = MatrixBinOp<View<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<View<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		Slice<T>
		{
			type Output = MatrixBinOp<Slice<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<Slice<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		Reshape<T>
		{
			type Output = MatrixBinOp<Reshape<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<Reshape<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		Transposer<T>
		{
			type Output = MatrixBinOp<Transposer<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<Transposer<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		RowAccessor<T>
		{
			type Output = MatrixBinOp<RowAccessor<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<RowAccessor<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T:   MatrixShape + Clone>
		$op_name<RHS> for
		ColumnAccessor<T>
		{
			type Output = MatrixBinOp<ColumnAccessor<T>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<ColumnAccessor<T>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T1:  MatrixShape + Clone,
		     T2:  MatrixShape + Clone>
		$op_name<RHS> for
		MatrixMul<T1, T2>
		{
			type Output = MatrixBinOp<MatrixMul<T1, T2>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<MatrixMul<T1, T2>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T1:  MatrixShape + Clone,
		     T2:  MatrixShape + Clone>
		$op_name<RHS> for
		HStack<T1, T2>
		{
			type Output = MatrixBinOp<HStack<T1, T2>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<HStack<T1, T2>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: MatrixRawGet + Clone + SameShape,
		     T1:  MatrixShape + Clone,
		     T2:  MatrixShape + Clone>
		$op_name<RHS> for
		VStack<T1, T2>
		{
			type Output = MatrixBinOp<VStack<T1, T2>, RHS, $op>;
			fn $op_method(self, rhs: RHS) -> MatrixBinOp<VStack<T1, T2>, RHS, $op>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
	}
}

bin_op!(Add, add, OpAdd);
bin_op!(Sub, sub, OpSub);
bin_op!(Mul, mul, OpMul);
bin_op!(Div, div, OpDiv);
