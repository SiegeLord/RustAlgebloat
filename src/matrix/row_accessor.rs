use std::fmt;

use matrix::traits::{MatrixGet, MatrixShape, MatrixRowAccess};
use vector::traits::{VectorGet, LengthEq};
use vector::write_vec;

pub struct RowAccessor<T>
{
	base: T,
	row: uint
}

impl<T: MatrixShape>
RowAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, row: uint) -> RowAccessor<T>
	{
		RowAccessor{ base: base, row: row }
	}
	
	pub fn new(base: T, row: uint) -> RowAccessor<T>
	{
		assert!(row < base.nrow());
		RowAccessor{ base: base, row: row }
	}
}

impl<T: MatrixShape>
LengthEq for
RowAccessor<T>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<'l,
     T: MatrixGet + MatrixShape>
VectorGet for
RowAccessor<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		self.base.unsafe_get(self.row, idx)
	}

	fn get(&self, idx: uint) -> f64
	{
		assert!(idx < self.base.ncol());
		unsafe
		{
			self.base.unsafe_get(self.row, idx)
		}
	}
}

impl<T: MatrixShape>
Container for
RowAccessor<T>
{
	fn len(&self) -> uint
	{
		self.base.ncol()
	}
}

impl<T: MatrixShape + MatrixGet>
fmt::Show for
RowAccessor<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, self)
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
