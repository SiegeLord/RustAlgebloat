// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::ops::{Neg};

use matrix::traits::{MatrixShape, MatrixGet};
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use vector::Vector;
use vector::write_vec;
use vector::traits::*;
use vector::slice::Slice;
use vector::bin_ops::{VectorBinOp, BinOp};

pub trait UnOp
{
	fn op(&self, a: f32) -> f32;
}

#[deriving(Clone)]
pub struct OpNeg;
impl OpNeg
{
	pub fn new() -> OpNeg
	{
		OpNeg
	}
}
impl UnOp for OpNeg
{
	fn op(&self, a: f32) -> f32
	{
		-a
	}
}

pub struct VectorUnOp<TA, TO>
{
	priv a: TA,
	priv o: TO
}

impl<TA,
     TO: UnOp>
VectorUnOp<TA, TO>
{
	pub fn new(a: TA, o: TO) -> VectorUnOp<TA, TO>
	{
		VectorUnOp{ a: a, o: o }
	}
}

impl<TA: Clone + Container,
     TO: UnOp>
VectorSlice for
VectorUnOp<TA, TO>
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<VectorUnOp<TA, TO>>
	{
		Slice::unsafe_new(self, start, end)
	}

	fn slice(self, start: uint, end: uint) -> Slice<VectorUnOp<TA, TO>>
	{
		Slice::new(self, start, end)
	}
}

impl<TA: Clone,
     TO: Clone>
Clone for
VectorUnOp<TA, TO>
{
	fn clone(&self) -> VectorUnOp<TA, TO>
	{
		VectorUnOp{ a: self.a.clone(), o: self.o.clone() }
	}
}

impl<TA: VectorGet + Container,
     TO: UnOp>
VectorGet for
VectorUnOp<TA, TO>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.o.op(self.a.unsafe_get(idx))
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.o.op(self.a.get(idx))
	}
}

impl<TA: Container,
     TO>
Container for
VectorUnOp<TA, TO>
{
	fn len(&self) -> uint
	{
		self.a.len()
	}
}

impl<TA: Container,
     TO: UnOp>
LengthEq for
VectorUnOp<TA, TO>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<TA: VectorGet + Container,
     TO: UnOp>
fmt::Show for
VectorUnOp<TA, TO>
{
	fn fmt(v: &VectorUnOp<TA, TO>, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, v)
	}
}

impl<TA: VectorGet + Clone + Container,
     TO: UnOp + Clone>
Neg<VectorUnOp<VectorUnOp<TA, TO>, OpNeg>> for
VectorUnOp<TA, TO>
{
	fn neg(&self) -> VectorUnOp<VectorUnOp<TA, TO>, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<TA: VectorGet + Clone + Container,
     TB: Clone,
     TO: BinOp + Clone>
Neg<VectorUnOp<VectorBinOp<TA, TB, TO>, OpNeg>> for
VectorBinOp<TA, TB, TO>
{
	fn neg(&self) -> VectorUnOp<VectorBinOp<TA, TB, TO>, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<'l>
Neg<VectorUnOp<&'l Vector, OpNeg>> for
&'l Vector
{
	fn neg(&self) -> VectorUnOp<&'l Vector, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: Clone>
Neg<VectorUnOp<Slice<T>, OpNeg>> for
Slice<T>
{
	fn neg(&self) -> VectorUnOp<Slice<T>, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<VectorUnOp<RowAccessor<T>, OpNeg>> for
RowAccessor<T>
{
	fn neg(&self) -> VectorUnOp<RowAccessor<T>, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<VectorUnOp<ColumnAccessor<T>, OpNeg>> for
ColumnAccessor<T>
{
	fn neg(&self) -> VectorUnOp<ColumnAccessor<T>, OpNeg>
	{
		VectorUnOp::new(self.clone(), OpNeg::new())
	}
}
