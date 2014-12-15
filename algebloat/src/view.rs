use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixView, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixView for
T
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		View::unsafe_new(self, row_start, col_start, row_end, col_end)
	}

	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		View::new(self, row_start, col_start, row_end, col_end)
	}
}

#[deriving(Copy)]
pub struct View<T>
{
	base: T,
	row_start: uint,
	col_start: uint,
	row_end: uint,
	col_end: uint,
}

impl<T: MatrixShape>
View<T>
{
	pub unsafe fn unsafe_new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}

	pub fn new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		assert!(row_start <= row_end);
		assert!(col_start <= col_end);
		assert!(row_end <= base.nrow());
		assert!(col_end <= base.ncol());
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}
}

impl<T: MatrixRawGet>
MatrixRawGet for
View<T>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.base.raw_get(r + self.row_start, c + self.col_start)
	}
}

impl<T: MatrixRawSet>
MatrixRawSet for
View<T>
{
	unsafe fn raw_set(&self, r: uint, c: uint, val: f64)
	{
		self.base.raw_set(r + self.row_start, c + self.col_start, val)
	}
}

impl<T>
MatrixShape for
View<T>
{
	fn nrow(&self) -> uint
	{
		self.row_end - self.row_start
	}
	fn ncol(&self) -> uint
	{
		self.col_end - self.col_start
	}
}

impl<T: MatrixShape>
SameShape for
View<T>
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: Clone>
Clone for
View<T>
{
	fn clone(&self) -> View<T>
	{
		View{ base: self.base.clone(), row_start: self.row_start, col_start: self.col_start, row_end: self.row_end, col_end: self.col_end }
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Show for
View<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
