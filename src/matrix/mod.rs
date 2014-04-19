// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::vec::Vec;
use std::fmt;
use std::io::Writer;
use std::cell::Cell;

use matrix::traits::{MatrixGet, MatrixSet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixView, MatrixTranspose};
use matrix::transpose::Transposer;
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use matrix::view::View;

use safe_alias::SafeAlias;

pub mod traits;
pub mod transpose;
pub mod view;
pub mod row_accessor;
pub mod column_accessor;
#[cfg(test)]
mod test;

pub struct Matrix
{
	data: Vec<Cell<f32>>,
	nrow: uint,
	ncol: uint
}

impl Matrix
{
	pub fn new(data: &[&[f32]]) -> Matrix
	{
		let nrow = data.len();
		let ncol = data[0].len();
		let mut mat_data = Vec::with_capacity(nrow * ncol);
		for &row in data.iter()
		{
			assert!(row.len() == ncol);
			for &val in row.iter()
			{
				mat_data.push(Cell::new(val));
			}
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}

	pub fn from_elem(nrow: uint, ncol: uint, elem: f32) -> Matrix
	{
		Matrix{ data: Vec::from_elem(nrow * ncol, Cell::new(elem)), nrow: nrow, ncol: ncol }
	}
}

impl<'l>
MatrixGet for
&'l Matrix
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		self.data.as_slice().unsafe_ref(c + r * self.ncol).get()
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		unsafe
		{
			self.unsafe_get(r, c)
		}
	}
}

impl<'l>
MatrixSet for
&'l Matrix
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f32)
	{
		self.data.as_slice().unsafe_ref(c + r * self.ncol).set(val);
	}

	fn set(&self, r: uint, c: uint, val: f32)
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		unsafe
		{
			self.unsafe_set(r, c, val);
		}
	}
}

impl<'l>
MatrixShape for
&'l Matrix
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

impl
MatrixGet for
Matrix
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		self.data.as_slice().unsafe_ref(c + r * self.ncol).get()
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		unsafe
		{
			self.unsafe_get(r, c)
		}
	}
}

impl
MatrixSet for
Matrix
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f32)
	{
		self.data.as_slice().unsafe_ref(c + r * self.ncol).set(val);
	}

	fn set(&self, r: uint, c: uint, val: f32)
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		unsafe
		{
			self.unsafe_set(r, c, val);
		}
	}
}

impl
MatrixShape for
Matrix
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

impl<'l>
MatrixTranspose for
&'l Matrix
{
	fn t(self) -> Transposer<&'l Matrix>
	{
		Transposer::new(self)
	}
}

impl<'l>
MatrixRowAccess for
&'l Matrix
{
	unsafe fn unsafe_row(self, row: uint) -> RowAccessor<&'l Matrix>
	{
		RowAccessor::unsafe_new(self, row)
	}
	
	fn row(self, row: uint) -> RowAccessor<&'l Matrix>
	{
		RowAccessor::new(self, row)
	}
}

impl<'l>
MatrixColumnAccess for
&'l Matrix
{
	unsafe fn unsafe_col(self, col: uint) -> ColumnAccessor<&'l Matrix>
	{
		ColumnAccessor::unsafe_new(self, col)
	}
	
	fn col(self, col: uint) -> ColumnAccessor<&'l Matrix>
	{
		ColumnAccessor::new(self, col)
	}
}

impl<'l>
MatrixView for
&'l Matrix
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<&'l Matrix>
	{
		View::unsafe_new(self, row_start, col_start, row_end, col_end)
	}

	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<&'l Matrix>
	{
		View::new(self, row_start, col_start, row_end, col_end)
	}
}

pub fn write_mat<T: MatrixGet + MatrixShape>(w: &mut Writer, a: &T) -> fmt::Result
{
	for r in range(0, a.nrow())
	{
		let mut first = true;
		try!(write!(w, "│"))
		for c in range(0, a.ncol())
		{
			if !first
			{
				try!(write!(w, " "))
			}
			first = false;
			unsafe
			{
				try!(write!(w, "{}", a.unsafe_get(r, c)))
			}
		}
		try!(write!(w, "│"))
		if r + 1 < a.nrow()
		{
			try!(writeln!(w, ""))
		}
	}
	Ok(())
}

impl
fmt::Show for
Matrix
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
	}
}

impl
SafeAlias for
Matrix {}

impl<'l>
SafeAlias for
&'l Matrix {}
