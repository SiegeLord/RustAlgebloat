use std::fmt;

use traits::{MatrixRawGet, MatrixGet, MatrixSet, MatrixRawSet, MatrixShape, MatrixSlice, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixSlice for
T
{
	unsafe fn unsafe_slice(self, start: usize, end: usize) -> Slice<T>
	{
		Slice::unsafe_new(self, start, end)
	}

	fn slice(self, start: usize, end: usize) -> Slice<T>
	{
		Slice::new(self, start, end)
	}
}

#[derive(Copy)]
pub struct Slice<T>
{
	base: T,
	start: usize,
	end: usize,
}

impl<T: MatrixShape>
Slice<T>
{
	pub unsafe fn unsafe_new(base: T, start: usize, end: usize) -> Slice<T>
	{
		Slice{ base: base, start: start, end: end }
	}

	pub fn new(base: T, start: usize, end: usize) -> Slice<T>
	{
		assert!(start <= end);
		assert!(end <= base.len());
		Slice{ base: base, start: start, end: end }
	}
}

impl<T: MatrixGet<usize>>
MatrixRawGet for
Slice<T>
{
	unsafe fn raw_get(&self, r: usize, _c: usize) -> f64
	{
		self.base.unsafe_get(r + self.start)
	}
}

impl<T: MatrixSet<usize>>
MatrixRawSet for
Slice<T>
{
	unsafe fn raw_set(&self, r: usize, _c: usize, val: f64)
	{
		self.base.unsafe_set(r + self.start, val)
	}
}

impl<T>
MatrixShape for
Slice<T>
{
	fn nrow(&self) -> usize
	{
		self.end - self.start
	}
	fn ncol(&self) -> usize
	{
		1
	}
}

impl<T: MatrixShape>
SameShape for
Slice<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: Clone>
Clone for
Slice<T>
{
	fn clone(&self) -> Slice<T>
	{
		Slice{ base: self.base.clone(), start: self.start, end: self.end }
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Display for
Slice<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
