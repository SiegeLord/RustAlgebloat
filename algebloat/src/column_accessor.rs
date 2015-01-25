use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixColumnAccess, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixColumnAccess for
T
{
	unsafe fn unsafe_col(self, c: usize) -> ColumnAccessor<T>
	{
		ColumnAccessor::unsafe_new(self, c)
	}
	
	fn col(self, c: usize) -> ColumnAccessor<T>
	{
		ColumnAccessor::new(self, c)
	}
}

#[derive(Copy)]
pub struct ColumnAccessor<T>
{
	base: T,
	col: usize
}

impl<T: MatrixShape>
ColumnAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, col: usize) -> ColumnAccessor<T>
	{
		ColumnAccessor{ base: base, col: col }
	}

	pub fn new(base: T, col: usize) -> ColumnAccessor<T>
	{
		assert!(col < base.ncol());
		ColumnAccessor{ base: base, col: col }
	}
}

impl<T: MatrixShape>
MatrixShape for
ColumnAccessor<T>
{
	fn nrow(&self) -> usize
	{
		self.base.nrow()
	}
	fn ncol(&self) -> usize
	{
		1
	}
}

impl<T: MatrixShape>
SameShape for
ColumnAccessor<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: MatrixRawGet + MatrixShape>
MatrixRawGet for
ColumnAccessor<T>
{
	unsafe fn raw_get(&self, r: usize, _: usize) -> f64
	{
		self.base.raw_get(r, self.col)
	}
}

impl<T: MatrixRawSet + MatrixShape>
MatrixRawSet for
ColumnAccessor<T>
{
	unsafe fn raw_set(&self, r: usize, _: usize, v: f64)
	{
		self.base.raw_set(r, self.col, v)
	}
}

impl<T: MatrixShape + MatrixRawGet>
fmt::Display for
ColumnAccessor<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}

impl<T: Clone>
Clone for
ColumnAccessor<T>
{
	fn clone(&self) -> ColumnAccessor<T>
	{
		ColumnAccessor{ base: self.base.clone(), col: self.col }
	}
}
