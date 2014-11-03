// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::ops::{Neg};

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
use bin_ops::{MatrixBinOp, BinOp};

pub trait UnOp
{
	fn op(&self, a: f64) -> f64;
}

#[deriving(Clone)]
pub struct OpNeg;
impl OpNeg
{
	#[inline]
	pub fn new() -> OpNeg
	{
		OpNeg
	}
}
impl UnOp for OpNeg
{
	#[inline]
	fn op(&self, a: f64) -> f64
	{
		-a
	}
}

pub struct MatrixUnOp<TA, TO>
{
	a: TA,
	o: TO
}

impl<TA,
     TO: UnOp>
MatrixUnOp<TA, TO>
{
	pub fn new(a: TA, o: TO) -> MatrixUnOp<TA, TO>
	{
		MatrixUnOp{ a: a, o: o }
	}
}

impl<TA: Clone,
     TO: Clone>
Clone for
MatrixUnOp<TA, TO>
{
	fn clone(&self) -> MatrixUnOp<TA, TO>
	{
		MatrixUnOp{ a: self.a.clone(), o: self.o.clone() }
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TO: UnOp>
MatrixRawGet for
MatrixUnOp<TA, TO>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.o.op(self.a.raw_get(r, c))
	}
}

impl<TA: MatrixShape,
     TO: UnOp>
MatrixShape for
MatrixUnOp<TA, TO>
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

impl<TA: MatrixShape,
     TO: UnOp>
SameShape for
MatrixUnOp<TA, TO>
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TO: UnOp>
fmt::Show for
MatrixUnOp<TA, TO>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}

impl<TA: MatrixRawGet + Clone + MatrixShape,
     TO: UnOp + Clone>
Neg<MatrixUnOp<MatrixUnOp<TA, TO>, OpNeg>> for
MatrixUnOp<TA, TO>
{
	fn neg(&self) -> MatrixUnOp<MatrixUnOp<TA, TO>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<TA: MatrixRawGet + Clone + MatrixShape,
     TB: Clone,
     TO: BinOp + Clone>
Neg<MatrixUnOp<MatrixBinOp<TA, TB, TO>, OpNeg>> for
MatrixBinOp<TA, TB, TO>
{
	fn neg(&self) -> MatrixUnOp<MatrixBinOp<TA, TB, TO>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<'l>
Neg<MatrixUnOp<&'l Matrix, OpNeg>> for
&'l Matrix
{
	fn neg(&self) -> MatrixUnOp<&'l Matrix, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<RowAccessor<T>, OpNeg>> for
RowAccessor<T>
{
	fn neg(&self) -> MatrixUnOp<RowAccessor<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<ColumnAccessor<T>, OpNeg>> for
ColumnAccessor<T>
{
	fn neg(&self) -> MatrixUnOp<ColumnAccessor<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<Transposer<T>, OpNeg>> for
Transposer<T>
{
	fn neg(&self) -> MatrixUnOp<Transposer<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<View<T>, OpNeg>> for
View<T>
{
	fn neg(&self) -> MatrixUnOp<View<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<Slice<T>, OpNeg>> for
Slice<T>
{
	fn neg(&self) -> MatrixUnOp<Slice<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg<MatrixUnOp<Reshape<T>, OpNeg>> for
Reshape<T>
{
	fn neg(&self) -> MatrixUnOp<Reshape<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg<MatrixUnOp<MatrixMul<T1, T2>, OpNeg>> for
MatrixMul<T1, T2>
{
	fn neg(&self) -> MatrixUnOp<MatrixMul<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg<MatrixUnOp<HStack<T1, T2>, OpNeg>> for
HStack<T1, T2>
{
	fn neg(&self) -> MatrixUnOp<HStack<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg<MatrixUnOp<VStack<T1, T2>, OpNeg>> for
VStack<T1, T2>
{
	fn neg(&self) -> MatrixUnOp<VStack<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}
