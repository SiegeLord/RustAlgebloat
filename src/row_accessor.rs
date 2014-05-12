use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixRowAccess};
use matrix::write_mat;

impl<T: MatrixShape>
MatrixRowAccess for
T
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<T>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<T>
	{
		RowAccessor::new(self, r)
	}
}

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
MatrixShape for
RowAccessor<T>
{
	fn nrow(&self) -> uint
	{
		1
	}
	fn ncol(&self) -> uint
	{
		self.base.ncol()
	}
}

//~ impl<T: MatrixShape>
//~ LengthEq for
//~ RowAccessor<T>
//~ {
	//~ fn len_eq(&self, other_len: uint) -> bool
	//~ {
		//~ other_len == self.len()
	//~ }
//~ }

impl<T: MatrixRawGet + MatrixShape>
MatrixRawGet for
RowAccessor<T>
{
	unsafe fn raw_get(&self, _: uint, c: uint) -> f64
	{
		self.base.raw_get(self.row, c)
	}
}

impl<T: MatrixRawSet + MatrixShape>
MatrixRawSet for
RowAccessor<T>
{
	unsafe fn raw_set(&self, _: uint, c: uint, v: f64)
	{
		self.base.raw_set(self.row, c, v)
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

impl<T: MatrixShape + MatrixRawGet>
fmt::Show for
RowAccessor<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
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
