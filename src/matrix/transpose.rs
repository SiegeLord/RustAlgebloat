use std::fmt;

use matrix::traits::{MatrixGet, MatrixSet, MatrixShape/*, MatrixRowAccess, MatrixColumnAccess*/, MatrixView, MatrixTranspose/*, MatrixFlat*/};
use matrix::write_mat;
//~ use matrix::row_accessor::RowAccessor;
//~ use matrix::column_accessor::ColumnAccessor;
//~ use matrix::flat_view::FlatView;
use matrix::view::View;

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

impl<T: MatrixGet>
MatrixGet for
Transposer<T>
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f64
	{
		self.base.unsafe_get(c, r)
	}

	fn get(&self, r: uint, c: uint) -> f64
	{
		self.base.get(c, r)
	}
}

impl<T: MatrixSet>
MatrixSet for
Transposer<T>
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f64)
	{
		self.base.unsafe_set(c, r, val)
	}

	fn set(&self, r: uint, c: uint, val: f64)
	{
		self.base.set(c, r, val)
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

impl<T>
MatrixTranspose for
Transposer<T>
{
	fn t(self) -> Transposer<Transposer<T>>
	{
		Transposer{ base: self }
	}
}

/*impl<T: MatrixShape>
MatrixFlat for
Transposer<T>
{
	fn flat(self) -> FlatView<Transposer<T>>
	{
		FlatView::new(self)
	}
}

impl<T: MatrixShape>
MatrixColumnAccess for
Transposer<T>
{
	unsafe fn unsafe_col(self, c: uint) -> ColumnAccessor<Transposer<T>>
	{
		ColumnAccessor::unsafe_new(self, c)
	}
	
	fn col(self, c: uint) -> ColumnAccessor<Transposer<T>>
	{
		ColumnAccessor::new(self, c)
	}
}

impl<T: MatrixShape>
MatrixRowAccess for
Transposer<T>
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<Transposer<T>>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<Transposer<T>>
	{
		RowAccessor::new(self, r)
	}
}*/

impl<T: MatrixShape>
MatrixView for
Transposer<T>
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Transposer<T>>
	{
		View::unsafe_new(self, row_start, col_start, row_end, col_end)
	}

	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Transposer<T>>
	{
		View::new(self, row_start, col_start, row_end, col_end)
	}
}

impl<T: MatrixGet + MatrixShape>
fmt::Show for
Transposer<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
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
