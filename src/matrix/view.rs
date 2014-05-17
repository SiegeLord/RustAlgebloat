use std::fmt;

use matrix::traits::{MatrixGet, MatrixSet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixView, MatrixTranspose, MatrixFlat};
use matrix::transpose::Transposer;
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use matrix::flat_view::FlatView;
use matrix::write_mat;

pub struct View<T>
{
	base: T,
	row_start: uint,
	col_start: uint,
	row_end: uint,
	col_end: uint,
}

impl<T: MatrixGet>
MatrixGet for
View<T>
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f64
	{
		self.base.unsafe_get(r + self.row_start, c + self.col_start)
	}

	fn get(&self, r: uint, c: uint) -> f64
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		self.base.get(r + self.row_start, c + self.col_start)
	}
}

impl<T: MatrixSet>
MatrixSet for
View<T>
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f64)
	{
		self.base.unsafe_set(r + self.row_start, c + self.col_start, val)
	}

	fn set(&self, r: uint, c: uint, val: f64)
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		self.base.set(r + self.row_start, c + self.col_start, val)
	}
}

impl<T: MatrixShape>
View<T>
{
	pub unsafe fn unsafe_new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}

	pub fn new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		assert!(row_start <= row_end);
		assert!(col_start <= col_end);
		assert!(row_end <= base.nrow());
		assert!(col_end <= base.ncol());
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}
}

impl<T: MatrixShape>
MatrixView for
View<T>
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<View<T>>
	{
		View::unsafe_new(self, row_start, col_start, row_end, col_end)
	}

	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<View<T>>
	{
		View::new(self, row_start, col_start, row_end, col_end)
	}
}

impl<T>
MatrixShape for
View<T>
{
	fn nrow(&self) -> uint
	{
		self.row_end - self.row_start
	}
	fn ncol(&self) -> uint
	{
		self.col_end - self.col_start
	}
}

impl<T: Clone>
Clone for
View<T>
{
	fn clone(&self) -> View<T>
	{
		View{ base: self.base.clone(), row_start: self.row_start, col_start: self.col_start, row_end: self.row_end, col_end: self.col_end }
	}
}

impl<T: MatrixShape>
MatrixFlat for
View<T>
{
	fn flat(self) -> FlatView<View<T>>
	{
		FlatView::new(self)
	}
}

impl<T: MatrixShape>
MatrixColumnAccess for
View<T>
{
	unsafe fn unsafe_col(self, c: uint) -> ColumnAccessor<View<T>>
	{
		ColumnAccessor::unsafe_new(self, c)
	}
	
	fn col(self, c: uint) -> ColumnAccessor<View<T>>
	{
		ColumnAccessor::new(self, c)
	}
}

impl<T: MatrixShape>
MatrixRowAccess for
View<T>
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<View<T>>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<View<T>>
	{
		RowAccessor::new(self, r)
	}
}

impl<T>
MatrixTranspose for
View<T>
{
	fn t(self) -> Transposer<View<T>>
	{
		Transposer::new(self)
	}
}

impl<T: MatrixGet + MatrixShape>
fmt::Show for
View<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
