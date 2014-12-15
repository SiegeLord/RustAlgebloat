use std::fmt;

use traits::{MatrixGet, MatrixSet, MatrixRawGet, MatrixRawSet, MatrixShape, MatrixReshape, SameShape};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixReshape for
T
{
	unsafe fn unsafe_reshape(self, nrow: uint, ncol: uint) -> Reshape<T>
	{
		Reshape::unsafe_new(self, nrow, ncol)
	}

	fn reshape(self, nrow: uint, ncol: uint) -> Reshape<T>
	{
		Reshape::new(self, nrow, ncol)
	}
}

#[deriving(Copy)]
pub struct Reshape<T>
{
	base: T,
	nrow: uint,
	ncol: uint,
}

impl<T: MatrixShape>
Reshape<T>
{
	pub unsafe fn unsafe_new(base: T, nrow: uint, ncol: uint) -> Reshape<T>
	{
		Reshape{ base: base, nrow: nrow, ncol: ncol }
	}

	pub fn new(base: T, nrow: uint, ncol: uint) -> Reshape<T>
	{
		assert!(nrow * ncol == base.len());
		Reshape{ base: base, nrow: nrow, ncol: ncol }
	}
}

impl<T: MatrixGet<uint>>
MatrixRawGet for
Reshape<T>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.base.unsafe_get(r * self.ncol() + c)
	}
}

impl<T: MatrixSet<uint>>
MatrixRawSet for
Reshape<T>
{
	unsafe fn raw_set(&self, r: uint, c: uint, val: f64)
	{
		self.base.unsafe_set(r * self.ncol() + c, val)
	}
}

impl<T>
MatrixShape for
Reshape<T>
{
	fn nrow(&self) -> uint
	{
		self.nrow
	}
	fn ncol(&self) -> uint
	{
		self.ncol
	}
}

impl<T: MatrixShape>
SameShape for
Reshape<T>
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
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
fmt::Show for
Reshape<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
