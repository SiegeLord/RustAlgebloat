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

#[derive(Copy, Clone)]
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

#[derive(Copy)]
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
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.o.op(self.a.raw_get(r, c))
	}
}

impl<TA: MatrixShape,
     TO: UnOp>
MatrixShape for
MatrixUnOp<TA, TO>
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
     TO: UnOp>
SameShape for
MatrixUnOp<TA, TO>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<TA: MatrixRawGet + MatrixShape,
     TO: UnOp>
fmt::Display for
MatrixUnOp<TA, TO>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}

impl<TA: MatrixRawGet + Clone + MatrixShape,
     TO: UnOp + Clone>
Neg for
MatrixUnOp<TA, TO>
{
	type Output = MatrixUnOp<MatrixUnOp<TA, TO>, OpNeg>;
	fn neg(self) -> MatrixUnOp<MatrixUnOp<TA, TO>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<TA: MatrixRawGet + Clone + MatrixShape,
     TB: Clone,
     TO: BinOp + Clone>
Neg for
MatrixBinOp<TA, TB, TO>
{
	type Output = MatrixUnOp<MatrixBinOp<TA, TB, TO>, OpNeg>;
	fn neg(self) -> MatrixUnOp<MatrixBinOp<TA, TB, TO>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<'l>
Neg for
&'l Matrix
{
	type Output = MatrixUnOp<&'l Matrix, OpNeg>;
	fn neg(self) -> MatrixUnOp<&'l Matrix, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
RowAccessor<T>
{
	type Output = MatrixUnOp<RowAccessor<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<RowAccessor<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
ColumnAccessor<T>
{
	type Output = MatrixUnOp<ColumnAccessor<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<ColumnAccessor<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
Transposer<T>
{
	type Output = MatrixUnOp<Transposer<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<Transposer<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
View<T>
{
	type Output = MatrixUnOp<View<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<View<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
Slice<T>
{
	type Output = MatrixUnOp<Slice<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<Slice<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T: MatrixShape + Clone>
Neg for
Reshape<T>
{
	type Output = MatrixUnOp<Reshape<T>, OpNeg>;
	fn neg(self) -> MatrixUnOp<Reshape<T>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg for
MatrixMul<T1, T2>
{
	type Output = MatrixUnOp<MatrixMul<T1, T2>, OpNeg>;
	fn neg(self) -> MatrixUnOp<MatrixMul<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg for
HStack<T1, T2>
{
	type Output = MatrixUnOp<HStack<T1, T2>, OpNeg>;
	fn neg(self) -> MatrixUnOp<HStack<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}

impl<T1: MatrixShape + Clone,
     T2: MatrixShape + Clone>
Neg for
VStack<T1, T2>
{
	type Output = MatrixUnOp<VStack<T1, T2>, OpNeg>;
	fn neg(self) -> MatrixUnOp<VStack<T1, T2>, OpNeg>
	{
		MatrixUnOp::new(self.clone(), OpNeg::new())
	}
}
