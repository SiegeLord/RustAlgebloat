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

impl<T: MatrixShape>
Collection for
Transposer<T>
{
	#[inline]
	fn len(&self) -> uint
	{
		self.nrow() * self.ncol()
	}
}

impl<T: MatrixRawGet>
MatrixRawGet for
Transposer<T>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.base.raw_get(c, r)
	}
}

impl<T: MatrixRawSet>
MatrixRawSet for
Transposer<T>
{
	unsafe fn raw_set(&self, r: uint, c: uint, val: f64)
	{
		self.base.raw_set(c, r, val)
	}
}

impl<T: MatrixShape>
MatrixShape for
Transposer<T>
{
	fn ncol(&self) -> uint
	{
		self.base.nrow()
	}

	fn nrow(&self) -> uint
	{
		self.base.ncol()
	}
}

impl<T: MatrixShape>
SameShape for
Transposer<T>
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Show for
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
