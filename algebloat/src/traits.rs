// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use transpose::Transposer;
use row_accessor::RowAccessor;
use column_accessor::ColumnAccessor;
use elements::MatrixElements;
use hstack::HStack;
use vstack::VStack;
use matrix_mul::MatrixMul;
use view::View;
use slice::Slice;
use reshape::Reshape;
use matrix::Matrix;
use index::{MatrixIndexGet, MatrixIndexSet};

pub trait MatrixRawGet
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64;
}

pub trait MatrixRawSet
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64);
}

pub trait MatrixGet<T>
{
	unsafe fn unsafe_get(&self, idx: T) -> f64;
	fn get(&self, idx: T) -> f64;
}

pub trait MatrixSet<T>
{
	unsafe fn unsafe_set(&self, idx: T, val: f64);
	fn set(&self, idx: T, val: f64);
}

pub trait MatrixShape
{
	fn ncol(&self) -> usize;
	fn nrow(&self) -> usize;
	#[inline]
	fn size(&self) -> (usize, usize)
	{
		(self.nrow(), self.ncol())
	}
	#[inline]
	fn len(&self) -> usize
	{
		self.nrow() * self.ncol()
	}
}

/* Hack necessary for operator overloading */
pub trait SameShape
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool;
}

pub trait MatrixTranspose
{
	fn t(self) -> Transposer<Self>;
}

pub trait MatrixRowAccess
{
	unsafe fn unsafe_row(self, row: usize) -> RowAccessor<Self>;
	fn row(self, row: usize) -> RowAccessor<Self>;
}

pub trait MatrixColumnAccess
{
	unsafe fn unsafe_col(self, col: usize) -> ColumnAccessor<Self>;
	fn col(self, col: usize) -> ColumnAccessor<Self>;
}

pub trait MatrixView
{
	unsafe fn unsafe_view(self, row_start: usize, col_start: usize, row_end: usize, col_end: usize) -> View<Self>;
	fn view(self, row_start: usize, col_start: usize, row_end: usize, col_end: usize) -> View<Self>;
}

pub trait MatrixReshape
{
	unsafe fn unsafe_reshape(self, nrow: usize, ncol: usize) -> Reshape<Self>;
	fn reshape(self, nrow: usize, ncol: usize) -> Reshape<Self>;
}

pub trait MatrixSlice
{
	unsafe fn unsafe_slice(self, start: usize, end: usize) -> Slice<Self>;
	fn slice(self, start: usize, end: usize) -> Slice<Self>;
}

pub trait MatrixAssign<RHS>
{
	unsafe fn unsafe_assign(&self, m: RHS);
	fn assign(&self, m: RHS);
}

pub trait MatrixMultiply<RHS>
{
	unsafe fn unsafe_mat_mul(self, rhs: RHS) -> Matrix;
	unsafe fn unsafe_mat_mul_lazy(self, rhs: RHS) -> MatrixMul<Self, RHS>;
	fn mat_mul(self, rhs: RHS) -> Matrix;
	fn mat_mul_lazy(self, rhs: RHS) -> MatrixMul<Self, RHS>;
}

pub trait MatrixHStack<R>
{
	unsafe fn unsafe_hstack(self, right: R) -> HStack<Self, R>;
	fn hstack(self, right: R) -> HStack<Self, R>;
}

pub trait MatrixVStack<B>
{
	unsafe fn unsafe_vstack(self, bot: B) -> VStack<Self, B>;
	fn vstack(self, bot: B) -> VStack<Self, B>;
}

pub trait MatrixElems
{
	fn elems(self) -> MatrixElements<Self>;
}

pub trait ToMatrix
{
	fn to_mat(self) -> Matrix;
}

impl<LHS: MatrixRawGet + MatrixShape,
     T: MatrixIndexGet<LHS>>
MatrixGet<T> for
LHS
{
	unsafe fn unsafe_get(&self, idx: T) -> f64
	{
		idx.unsafe_get_idx(self)
	}

	fn get(&self, idx: T) -> f64
	{
		idx.get_idx(self)
	}
}

impl<LHS: MatrixRawSet + MatrixShape,
     T: MatrixIndexSet<LHS>>
MatrixSet<T> for
LHS
{
	unsafe fn unsafe_set(&self, idx: T, val: f64)
	{
		idx.unsafe_set_idx(self, val);
	}

	fn set(&self, idx: T, val: f64)
	{
		idx.set_idx(self, val);
	}
}

//~ impl<LHS: MatrixShape + MatrixRawSet,
     //~ RHS: MatrixShape + MatrixRawGet>
impl<LHS: MatrixShape + MatrixRawSet + MatrixSet<usize>,
     RHS: MatrixShape + MatrixRawGet + MatrixGet<usize>>
MatrixAssign<RHS> for LHS
{
	unsafe fn unsafe_assign(&self, m: RHS)
	{
		//~ for r in 0..self.nrow()
		//~ {
			//~ for c in 0..self.ncol()
			//~ {
				//~ self.raw_set(r, c, m.raw_get(r, c));
			//~ }
		//~ }
		//~ 
		for i in 0..self.len()
		{
			self.unsafe_set(i, m.unsafe_get(i));
		}
	}
	
	fn assign(&self, m: RHS)
	{
		assert_eq!(self.nrow(), m.nrow());
		assert_eq!(self.ncol(), m.ncol());
		unsafe
		{
			self.unsafe_assign(m);
		}
	}
}

impl<T: MatrixShape + MatrixGet<usize>>
ToMatrix for T
{
	fn to_mat(self) -> Matrix
	{
		Matrix::from_iter(self.nrow(), self.ncol(), self.elems())
	}
}

// TODO: Re-add when the trait reform is implemented
//~ impl<T: MatrixShape>
//~ SameShape for
//~ T
//~ {
	//~ fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	//~ {
		//~ self.nrow() == nrow && self.ncol() == ncol
	//~ }
//~ }
