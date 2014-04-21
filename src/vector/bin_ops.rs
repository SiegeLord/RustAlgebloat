// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::ops::{Add, Sub, Mul};

use matrix::traits::{MatrixShape, MatrixGet};
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use vector::Vector;
use vector::write_vec;
use vector::traits::*;
use vector::slice::Slice;
use vector::un_ops::{VectorUnOp, UnOp};

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
			pub fn new() -> $name
			{
				$name
			}
		}
		impl BinOp for $name
		{
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

pub struct VectorBinOp<TA, TB, TO>
{
	a: TA,
	b: TB,
	o: TO
}

impl<TA: Container,
     TB: LengthEq,
     TO: BinOp>
VectorBinOp<TA, TB, TO>
{
	pub unsafe fn unsafe_new(a: TA, b: TB, o: TO) -> VectorBinOp<TA, TB, TO>
	{
		VectorBinOp{ a: a, b: b, o: o}
	}

	pub fn new(a: TA, b: TB, o: TO) -> VectorBinOp<TA, TB, TO>
	{
		assert!(b.len_eq(a.len()));
		VectorBinOp{ a: a, b: b, o: o }
	}
}

impl<TA: Clone + Container,
     TB: Clone + LengthEq,
     TO: BinOp>
VectorSlice for
VectorBinOp<TA, TB, TO>
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<VectorBinOp<TA, TB, TO>>
	{
		Slice::unsafe_new(self, start, end)
	}

	fn slice(self, start: uint, end: uint) -> Slice<VectorBinOp<TA, TB, TO>>
	{
		Slice::new(self, start, end)
	}
}

impl<TA: Clone,
     TB: Clone,
     TO: Clone>
Clone for
VectorBinOp<TA, TB, TO>
{
	fn clone(&self) -> VectorBinOp<TA, TB, TO>
	{
		VectorBinOp{ a: self.a.clone(), b: self.b.clone(), o: self.o.clone() }
	}
}

impl<TA: VectorGet + Container,
     TB: VectorGet + LengthEq,
     TO: BinOp>
VectorGet for
VectorBinOp<TA, TB, TO>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		self.o.op(self.a.unsafe_get(idx), self.b.unsafe_get(idx))
	}
	
	fn get(&self, idx: uint) -> f64
	{
		assert!(idx < self.len());
		unsafe
		{
			self.o.op(self.a.unsafe_get(idx), self.b.unsafe_get(idx))
		}
	}
}

impl<TA: Container,
	 TB,
     TO: BinOp>
Container for
VectorBinOp<TA, TB, TO>
{
	fn len(&self) -> uint
	{
		self.a.len()
	}
}

impl<TA: Container,
	 TB,
     TO: BinOp>
LengthEq for
VectorBinOp<TA, TB, TO>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<TA: VectorGet + Container,
     TB: VectorGet + LengthEq,
     TO: BinOp>
fmt::Show for
VectorBinOp<TA, TB, TO>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, self)
	}
}

macro_rules! bin_op
{
	($op_name: ident, $op_method: ident, $op: ident) =>
	{
		impl<RHS: VectorGet + Clone + LengthEq,
             TA: VectorGet + Clone + Container,
             TB: VectorGet + Clone + LengthEq,
             TO: BinOp + Clone>
		$op_name<RHS, VectorBinOp<VectorBinOp<TA, TB, TO>, RHS, $op>> for
		VectorBinOp<TA, TB, TO>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<VectorBinOp<TA, TB, TO>, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<RHS: VectorGet + Clone + LengthEq,
             TA: VectorGet + Clone + Container,
             TO: UnOp + Clone>
		$op_name<RHS, VectorBinOp<VectorUnOp<TA, TO>, RHS, $op>> for
		VectorUnOp<TA, TO>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<VectorUnOp<TA, TO>, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<'l,
		     RHS: VectorGet + Clone + LengthEq>
		$op_name<RHS, VectorBinOp<&'l Vector, RHS, $op>> for
		&'l Vector
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<&'l Vector, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   Clone>
		$op_name<RHS, VectorBinOp<Slice<T>, RHS, $op>> for
		Slice<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<Slice<T>, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   MatrixShape + Clone>
		$op_name<RHS, VectorBinOp<RowAccessor<T>, RHS, $op>> for
		RowAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<RowAccessor<T>, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   MatrixShape + Clone>
		$op_name<RHS, VectorBinOp<ColumnAccessor<T>, RHS, $op>> for
		ColumnAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorBinOp<ColumnAccessor<T>, RHS, $op>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
	}
}

bin_op!(Add, add, OpAdd)
bin_op!(Sub, sub, OpSub)
bin_op!(Mul, mul, OpMul)
bin_op!(Div, div, OpDiv)
