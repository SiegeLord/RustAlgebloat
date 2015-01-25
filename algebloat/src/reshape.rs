use std::fmt;

use traits::{MatrixGet, MatrixSet, MatrixRawGet, MatrixRawSet, MatrixShape, MatrixReshape, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixReshape for
T
{
	unsafe fn unsafe_reshape(self, nrow: usize, ncol: usize) -> Reshape<T>
	{
		Reshape::unsafe_new(self, nrow, ncol)
	}

	fn reshape(self, nrow: usize, ncol: usize) -> Reshape<T>
	{
		Reshape::new(self, nrow, ncol)
	}
}

#[derive(Copy)]
pub struct Reshape<T>
{
	base: T,
	nrow: usize,
	ncol: usize,
}

impl<T: MatrixShape>
Reshape<T>
{
	pub unsafe fn unsafe_new(base: T, nrow: usize, ncol: usize) -> Reshape<T>
	{
		Reshape{ base: base, nrow: nrow, ncol: ncol }
	}

	pub fn new(base: T, nrow: usize, ncol: usize) -> Reshape<T>
	{
		assert!(nrow * ncol == base.len());
		Reshape{ base: base, nrow: nrow, ncol: ncol }
	}
}

impl<T: MatrixGet<usize>>
MatrixRawGet for
Reshape<T>
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.base.unsafe_get(r * self.ncol() + c)
	}
}

impl<T: MatrixSet<usize>>
MatrixRawSet for
Reshape<T>
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		self.base.unsafe_set(r * self.ncol() + c, val)
	}
}

impl<T>
MatrixShape for
Reshape<T>
{
	fn nrow(&self) -> usize
	{
		self.nrow
	}
	fn ncol(&self) -> usize
	{
		self.ncol
	}
}

impl<T: MatrixShape>
SameShape for
Reshape<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: Clone>
Clone for
Reshape<T>
{
	fn clone(&self) -> Reshape<T>
	{
		Reshape{ base: self.base.clone(), nrow: self.nrow, ncol: self.ncol }
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Display for
Reshape<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
