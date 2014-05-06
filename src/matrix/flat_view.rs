use std::fmt;

use matrix::traits::{MatrixGet, MatrixShape};
use vector::traits::{VectorGet, LengthEq};
use vector::write_vec;

pub struct FlatView<T>
{
	base: T,
}

impl<T: MatrixShape>
FlatView<T>
{
	pub fn new(base: T) -> FlatView<T>
	{
		FlatView{ base: base }
	}
}

impl<T: MatrixShape>
LengthEq for
FlatView<T>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<'l,
     T: MatrixGet + MatrixShape>
VectorGet for
FlatView<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		self.base.unsafe_get(idx / self.base.ncol(), idx % self.base.ncol())
	}

	fn get(&self, idx: uint) -> f64
	{
		assert!(idx < self.len());
		unsafe
		{
			self.base.unsafe_get(idx / self.base.ncol(), idx % self.base.ncol())
		}
	}
}

impl<T: MatrixShape>
Container for
FlatView<T>
{
	fn len(&self) -> uint
	{
		self.base.nrow() * self.base.ncol()
	}
}

impl<T: MatrixShape + MatrixGet>
fmt::Show for
FlatView<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, self)
	}
}

impl<T: Clone>
Clone for
FlatView<T>
{
	fn clone(&self) -> FlatView<T>
	{
		FlatView{ base: self.base.clone() }
	}
}
