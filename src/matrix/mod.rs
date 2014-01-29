// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::vec;
use std::fmt;
use std::io::Writer;

use matrix::traits::{MatrixGet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixView, MatrixTranspose};
use matrix::transpose::Transposer;
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use matrix::view::View;

pub mod traits;
pub mod transpose;
pub mod view;
pub mod row_accessor;
pub mod column_accessor;
#[cfg(test)]
mod test;

pub struct Matrix
{
	priv data: ~[f32],
	priv nrow: uint,
	priv ncol: uint
}

impl Matrix
{
	pub fn new(data: &[&[f32]]) -> Matrix
	{
		let nrow = data.len();
		let ncol = data[0].len();
		let mut mat_data = vec::with_capacity(nrow * ncol);
		for &row in data.iter()
		{
			assert!(row.len() == ncol);
			mat_data = vec::append(mat_data, row);
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}
}

impl<'l>
MatrixGet for
&'l Matrix
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		*self.data.unsafe_ref(c + r * self.ncol)
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
		*self.data.unsafe_ref(c + r * self.ncol)
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

pub fn write_mat<T: MatrixGet + MatrixShape>(w: &mut Writer, a: &T)
{
	for r in range(0, a.nrow())
	{
		let mut first = true;
		write!(w, "│");
		for c in range(0, a.ncol())
		{
			if !first
			{
				write!(w, " ");
			}
			first = false;
			unsafe
			{
				write!(w, "{}", a.unsafe_get(r, c));
			}
		}
		write!(w, "│");
		if r + 1 < a.nrow()
		{
			writeln!(w, "");
		}
	}
}

impl
fmt::Default for
Matrix
{
	fn fmt(v: &Matrix, buf: &mut fmt::Formatter)
	{
		write_mat(buf.buf, &v);
	}
}
