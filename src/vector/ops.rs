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

pub trait Op
{
	fn op(&self, a: f32, b: f32) -> f32;
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
		impl Op for $name
		{
			fn op(&self, a: f32, b: f32) -> f32
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

pub struct VectorVectorBinOp<TA, TB, TO>
{
	priv a: TA,
	priv b: TB,
	priv o: TO
}

impl<TA: Container,
     TB: LengthEq,
     TO: Op>
VectorVectorBinOp<TA, TB, TO>
{
	pub unsafe fn unsafe_new(a: TA, b: TB, o: TO) -> VectorVectorBinOp<TA, TB, TO>
	{
		VectorVectorBinOp{ a: a, b: b, o: o}
	}

	pub fn new(a: TA, b: TB, o: TO) -> VectorVectorBinOp<TA, TB, TO>
	{
		assert!(b.len_eq(a.len()));
		VectorVectorBinOp{ a: a, b: b, o: o }
	}
}

impl<TA: Clone + Container,
     TB: Clone + LengthEq,
     TO: Op>
VectorSlice for
VectorVectorBinOp<TA, TB, TO>
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<VectorVectorBinOp<TA, TB, TO>>
	{
		Slice::unsafe_new(self, start, end)
	}

	fn slice(self, start: uint, end: uint) -> Slice<VectorVectorBinOp<TA, TB, TO>>
	{
		Slice::new(self, start, end)
	}
}

impl<TA: Clone,
     TB: Clone,
     TO: Clone>
Clone for
VectorVectorBinOp<TA, TB, TO>
{
	fn clone(&self) -> VectorVectorBinOp<TA, TB, TO>
	{
		VectorVectorBinOp{ a: self.a.clone(), b: self.b.clone(), o: self.o.clone() }
	}
}

impl<TA: VectorGet + Container,
     TB: VectorGet + LengthEq,
     TO: Op>
VectorGet for
VectorVectorBinOp<TA, TB, TO>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.o.op(self.a.unsafe_get(idx), self.b.unsafe_get(idx))
	}
	
	fn get(&self, idx: uint) -> f32
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
     TO: Op>
Container for
VectorVectorBinOp<TA, TB, TO>
{
	fn len(&self) -> uint
	{
		self.a.len()
	}
}

impl<TA: Container,
	 TB,
     TO: Op>
LengthEq for
VectorVectorBinOp<TA, TB, TO>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<TA: VectorGet + Container,
     TB: VectorGet + LengthEq,
     TO: Op>
fmt::Default for
VectorVectorBinOp<TA, TB, TO>
{
	fn fmt(v: &VectorVectorBinOp<TA, TB, TO>, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, v);
	}
}

macro_rules! bin_op
{
	($op_name: ident, $op_method: ident, $op: ident) =>
	{
		impl<RHS: VectorGet + Clone + LengthEq,
             TA: VectorGet + Clone + Container,
             TB: VectorGet + Clone + LengthEq,
             TO: Op + Clone>
		$op_name<RHS, VectorVectorBinOp<VectorVectorBinOp<TA, TB, TO>, RHS, $op>> for
		VectorVectorBinOp<TA, TB, TO>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorVectorBinOp<VectorVectorBinOp<TA, TB, TO>, RHS, $op>
			{
				VectorVectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
		
		impl<'l,
		     RHS: VectorGet + Clone + LengthEq>
		$op_name<RHS, VectorVectorBinOp<&'l Vector, RHS, $op>> for
		&'l Vector
		{
			fn $op_method(&self, rhs: &RHS) -> VectorVectorBinOp<&'l Vector, RHS, $op>
			{
				VectorVectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   Clone>
		$op_name<RHS, VectorVectorBinOp<Slice<T>, RHS, $op>> for
		Slice<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorVectorBinOp<Slice<T>, RHS, $op>
			{
				VectorVectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   MatrixShape + Clone>
		$op_name<RHS, VectorVectorBinOp<RowAccessor<T>, RHS, $op>> for
		RowAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorVectorBinOp<RowAccessor<T>, RHS, $op>
			{
				VectorVectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}

		impl<RHS: VectorGet + Clone + LengthEq,
		     T:   MatrixShape + Clone>
		$op_name<RHS, VectorVectorBinOp<ColumnAccessor<T>, RHS, $op>> for
		ColumnAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> VectorVectorBinOp<ColumnAccessor<T>, RHS, $op>
			{
				VectorVectorBinOp::new(self.clone(), rhs.clone(), $op::new())
			}
		}
	}
}

bin_op!(Add, add, OpAdd)
bin_op!(Sub, sub, OpSub)
bin_op!(Mul, mul, OpMul)
bin_op!(Div, div, OpDiv)
