use std::fmt;

use matrix::traits::{MatrixGet, MatrixShape, MatrixColumnAccess};
use vector::traits::{VectorGet, LengthEq};
use vector::write_vec;

pub struct ColumnAccessor<T>
{
	priv base: T,
	priv col: uint
}

impl<T: MatrixShape>
ColumnAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, col: uint) -> ColumnAccessor<T>
	{
		ColumnAccessor{ base: base, col: col }
	}
	
	pub fn new(base: T, col: uint) -> ColumnAccessor<T>
	{
		assert!(col < base.ncol());
		ColumnAccessor{ base: base, col: col }
	}
}

impl<T: MatrixShape>
LengthEq for
ColumnAccessor<T>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<'l,
     T: MatrixGet + MatrixShape>
VectorGet for
ColumnAccessor<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.base.unsafe_get(idx, self.col)
	}

	fn get(&self, idx: uint) -> f32
	{
		assert!(idx < self.base.ncol());
		unsafe
		{
			self.base.unsafe_get(idx, self.col)
		}
	}
}

impl<T: MatrixShape>
Container for
ColumnAccessor<T>
{
	fn len(&self) -> uint
	{
		self.base.nrow()
	}
}

impl<T: MatrixShape + MatrixGet>
fmt::Show for
ColumnAccessor<T>
{
	fn fmt(v: &ColumnAccessor<T>, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, v)
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
