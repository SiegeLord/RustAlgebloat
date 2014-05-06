// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use matrix::transpose::Transposer;
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use matrix::flat_view::FlatView;
use matrix::matrix_mul::MatrixMul;
use matrix::view::View;
use matrix::Matrix;
use vector::traits::VectorElems;

pub trait MatrixGet
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f64;
	fn get(&self, r: uint, c: uint) -> f64;
}

pub trait MatrixSet
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f64);
	fn set(&self, r: uint, c: uint, val: f64);
}

pub trait MatrixShape
{
	fn ncol(&self) -> uint;
	fn nrow(&self) -> uint;
}

pub trait MatrixTranspose
{
	fn t(self) -> Transposer<Self>;
}

pub trait MatrixRowAccess
{
	unsafe fn unsafe_row(self, row: uint) -> RowAccessor<Self>;
	fn row(self, row: uint) -> RowAccessor<Self>;
}

pub trait MatrixColumnAccess
{
	unsafe fn unsafe_col(self, col: uint) -> ColumnAccessor<Self>;
	fn col(self, col: uint) -> ColumnAccessor<Self>;
}

pub trait MatrixFlat
{
	fn flat(self) -> FlatView<Self>;
}

pub trait MatrixView
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
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

pub trait ToMatrix
{
	fn to_mat(self) -> Matrix;
}

impl<LHS: MatrixShape + MatrixSet,
     RHS: MatrixShape + MatrixGet>
MatrixAssign<RHS> for LHS
{
	unsafe fn unsafe_assign(&self, m: RHS)
	{
		for r in range(0, self.nrow())
		{
			for c in range(0, self.ncol())
			{
				self.unsafe_set(r, c, m.unsafe_get(r, c));
			}
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

impl<LHS: MatrixShape + MatrixGet,
     RHS: MatrixShape + MatrixGet>
MatrixMultiply<RHS> for LHS
{
	unsafe fn unsafe_mat_mul(self, rhs: RHS) -> Matrix
	{
		MatrixMul::unsafe_new(self, rhs).to_mat()
	}

	unsafe fn unsafe_mat_mul_lazy(self, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		MatrixMul::unsafe_new(self, rhs)
	}

	fn mat_mul(self, rhs: RHS) -> Matrix
	{
		MatrixMul::new(self, rhs).to_mat()
	}

	fn mat_mul_lazy(self, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		MatrixMul::new(self, rhs)
	}
}

impl<T: MatrixShape + MatrixGet + MatrixFlat>
ToMatrix for T
{
	fn to_mat(self) -> Matrix
	{
		Matrix::from_iter(self.nrow(), self.ncol(), self.flat().elems())
	}
}
