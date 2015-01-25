use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixTranspose, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixTranspose for
T
{
	fn t(self) -> Transposer<T>
	{
		Transposer::new(self)
	}
}

#[derive(Copy)]
pub struct Transposer<T>
{
	base: T
}

impl<T>
Transposer<T>
{
	pub fn new(base: T) -> Transposer<T>
	{
		Transposer{ base: base }
	}
}

impl<T: MatrixRawGet>
MatrixRawGet for
Transposer<T>
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.base.raw_get(c, r)
	}
}

impl<T: MatrixRawSet>
MatrixRawSet for
Transposer<T>
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		self.base.raw_set(c, r, val)
	}
}

impl<T: MatrixShape>
MatrixShape for
Transposer<T>
{
	fn ncol(&self) -> usize
	{
		self.base.nrow()
	}

	fn nrow(&self) -> usize
	{
		self.base.ncol()
	}
}

impl<T: MatrixShape>
SameShape for
Transposer<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Display for
Transposer<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
} 

impl<T: Clone>
Clone for
Transposer<T>
{
	fn clone(&self) -> Transposer<T>
	{
		Transposer::new(self.base.clone())
	}
}
