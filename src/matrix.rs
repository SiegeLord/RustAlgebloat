// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::vec;
use std::fmt;
use std::io::Writer;
use vector::{VectorGet, write_vec};

pub trait MatrixGet
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32;
	fn get(&self, r: uint, c: uint) -> f32;
}

pub trait MatrixShape
{
	fn ncol(&self) -> uint;
	fn nrow(&self) -> uint;
}

pub trait MatrixTranspose
{
	fn t(self) -> Transposer<Self>;
}

pub trait MatrixRowAccess
{
	unsafe fn unsafe_row(self, row: uint) -> RowAccessor<Self>;
	fn row(self, row: uint) -> RowAccessor<Self>;
}

pub trait MatrixColumnAccess
{
	unsafe fn unsafe_col(self, col: uint) -> ColumnAccessor<Self>;
	fn col(self, col: uint) -> ColumnAccessor<Self>;
}

pub trait MatrixView
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
}

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
		Transposer{ base: self }
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

pub struct Transposer<T>
{
	base: T
}

impl<T: MatrixGet>
MatrixGet for
Transposer<T>
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		self.base.unsafe_get(c, r)
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		self.base.get(c, r)
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
}

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

pub struct View<T>
{
	priv base: T,
	priv row_start: uint,
	priv col_start: uint,
	priv row_end: uint,
	priv col_end: uint,
}

impl<T: MatrixGet>
MatrixGet for
View<T>
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f32
	{
		self.base.unsafe_get(r + self.row_start, c + self.col_start)
	}

	fn get(&self, r: uint, c: uint) -> f32
	{
		assert!(r < self.nrow());
		assert!(c < self.ncol());
		self.base.get(r + self.row_start, c + self.col_start)
	}
}

impl<T: MatrixShape>
View<T>
{
	unsafe fn unsafe_new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
	{
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}

	fn new(base: T, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<T>
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
		Transposer{ base: self }
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

impl<'l,
     T: MatrixGet + MatrixShape>
VectorGet for
RowAccessor<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.base.unsafe_get(self.row, idx)
	}

	fn get(&self, idx: uint) -> f32
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
fmt::Default for
RowAccessor<T>
{
	fn fmt(v: &RowAccessor<T>, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, v);
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

pub struct ColumnAccessor<T>
{
	base: T,
	col: uint
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
fmt::Default for
ColumnAccessor<T>
{
	fn fmt(v: &ColumnAccessor<T>, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, v);
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

impl<T: MatrixGet + MatrixShape>
fmt::Default for
Transposer<T>
{
	fn fmt(v: &Transposer<T>, buf: &mut fmt::Formatter)
	{
		write_mat(buf.buf, v);
	}
} 

impl<T: Clone>
Clone for
Transposer<T>
{
	fn clone(&self) -> Transposer<T>
	{
		Transposer{ base: self.base.clone() }
	}
}

#[cfg(test)]
mod test
{
	extern mod extra;
	
	use vector::{VectorGet};
	
	use super::*;
	//~ use self::extra::test::BenchHarness;
	//~ use std::rand::{weak_rng, Rng};

	#[test]
	fn trans()
	{
		let m = Matrix::new([&[1.0, 2.0, 3.0],
						     &[4.0, 5.0, 6.0]]);
		assert_eq!(m.nrow(), 2);
		assert_eq!(m.ncol(), 3);
		let t = m.t();
		assert_eq!(t.nrow(), 3);
		assert_eq!(t.ncol(), 2);
		assert_eq!(m.get(1, 2), t.get(2, 1));
	}

	#[test]
	fn rows_and_cols()
	{
		let m = Matrix::new([&[1.0, 2.0, 3.0],
						     &[4.0, 5.0, 6.0],
	                         &[7.0, 8.0, 9.0]]);
		let v = m.row(0) + m.col(0);
		assert_eq!(v.get(1), 6.0);
	}
	
	#[test]
	fn views()
	{
		let m = Matrix::new([&[1.0, 2.0, 3.0],
						     &[4.0, 5.0, 6.0],
	                         &[7.0, 8.0, 9.0]]);
		let m1 = m.view(0, 0, m.nrow() - 1, m.ncol() - 1);
		let m2 = m.view(1, 1, m.nrow(), m.ncol());
		let v1 = m1.row(0) + m2.row(0);
		let v2 = m1.t().row(0) + m2.t().row(0);
		assert_eq!(v1.get(1), 8.0);
		assert_eq!(v2.get(1), 12.0);
	}
}
