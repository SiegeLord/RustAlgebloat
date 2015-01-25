use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixRowAccess, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixRowAccess for
T
{
	unsafe fn unsafe_row(self, r: usize) -> RowAccessor<T>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: usize) -> RowAccessor<T>
	{
		RowAccessor::new(self, r)
	}
}

#[derive(Copy)]
pub struct RowAccessor<T>
{
	base: T,
	row: usize
}

impl<T: MatrixShape>
RowAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, row: usize) -> RowAccessor<T>
	{
		RowAccessor{ base: base, row: row }
	}

	pub fn new(base: T, row: usize) -> RowAccessor<T>
	{
		assert!(row < base.nrow());
		RowAccessor{ base: base, row: row }
	}
}

impl<T: MatrixShape>
MatrixShape for
RowAccessor<T>
{
	fn nrow(&self) -> usize
	{
		1
	}
	fn ncol(&self) -> usize
	{
		self.base.ncol()
	}
}

impl<T: MatrixShape>
SameShape for
RowAccessor<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: MatrixRawGet + MatrixShape>
MatrixRawGet for
RowAccessor<T>
{
	unsafe fn raw_get(&self, _: usize, c: usize) -> f64
	{
		self.base.raw_get(self.row, c)
	}
}

impl<T: MatrixRawSet + MatrixShape>
MatrixRawSet for
RowAccessor<T>
{
	unsafe fn raw_set(&self, _: usize, c: usize, v: f64)
	{
		self.base.raw_set(self.row, c, v)
	}
}

impl<T: MatrixShape + MatrixRawGet>
fmt::Display for
RowAccessor<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}

impl<T: Clone>
Clone for
RowAccessor<T>
{
	fn clone(&self) -> RowAccessor<T>
	{
		RowAccessor{ base: self.base.clone(), row: self.row }
	}
}
