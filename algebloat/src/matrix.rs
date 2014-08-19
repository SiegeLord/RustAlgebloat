// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::vec::Vec;
use std::fmt;
use std::io::Writer;
use std::cell::Cell;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, SameShape};

pub struct Matrix
{
	data: Vec<Cell<f64>>,
	nrow: uint,
	ncol: uint
}

impl Matrix
{
	pub fn new(data: &[&[f64]]) -> Matrix
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

	pub fn eye(size: uint) -> Matrix
	{
		Matrix::from_fn(size, size, |r, c| if r == c { 1.0 } else { 0.0 })
	}

	pub fn zeros(nrow: uint, ncol: uint) -> Matrix
	{
		Matrix::from_elem(nrow, ncol, 0.0)
	}

	pub fn ones(nrow: uint, ncol: uint) -> Matrix
	{
		Matrix::from_elem(nrow, ncol, 1.0)
	}

	pub fn from_elem(nrow: uint, ncol: uint, elem: f64) -> Matrix
	{
		Matrix{ data: Vec::from_elem(nrow * ncol, Cell::new(elem)), nrow: nrow, ncol: ncol }
	}

	pub fn from_fn(nrow: uint, ncol: uint, cb: |uint, uint| -> f64) -> Matrix
	{
		let mut mat_data = Vec::with_capacity(nrow * ncol);
		for r in range(0, nrow)
		{
			for c in range(0, ncol)
			{
				mat_data.push(Cell::new(cb(r, c)));
			}
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}

	pub fn from_iter<T: Iterator<f64>>(nrow: uint, ncol: uint, t: T) -> Matrix
	{
		let mat_data: Vec<Cell<f64>> = t.map(|v| Cell::new(v)).collect();
		assert_eq!(mat_data.len(), nrow * ncol);
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}
}

impl<'l>
Collection for
&'l Matrix
{
	#[inline]
	fn len(&self) -> uint
	{
		self.nrow() * self.ncol()
	}
}

impl<'l>
MatrixRawGet for
&'l Matrix
{
	#[inline]
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.data.as_slice().unsafe_get(c + r * self.ncol).get()
	}
}

impl<'l>
MatrixRawSet for
&'l Matrix
{
	#[inline]
	unsafe fn raw_set(&self, r: uint, c: uint, val: f64)
	{
		self.data.as_slice().unsafe_get(c + r * self.ncol).set(val);
	}
}

impl<'l>
MatrixShape for
&'l Matrix
{
	#[inline]
	fn nrow(&self) -> uint
	{
		self.nrow
	}

	#[inline]
	fn ncol(&self) -> uint
	{
		self.ncol
	}
}

impl<'l>
SameShape for
&'l Matrix
{
	fn same_shape(&self, nrow: uint, ncol: uint) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl
Collection for
Matrix
{
	#[inline]
	fn len(&self) -> uint
	{
		self.nrow() * self.ncol()
	}
}

impl
MatrixRawGet for
Matrix
{
	#[inline]
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		self.data.as_slice().unsafe_get(c + r * self.ncol).get()
	}
}

impl
MatrixRawSet for
Matrix
{
	#[inline]
	unsafe fn raw_set(&self, r: uint, c: uint, val: f64)
	{
		self.data.as_slice().unsafe_get(c + r * self.ncol).set(val);
	}
}

impl
MatrixShape for
Matrix
{
	#[inline]
	fn nrow(&self) -> uint
	{
		self.nrow
	}

	#[inline]
	fn ncol(&self) -> uint
	{
		self.ncol
	}
}

pub fn write_mat<T: MatrixRawGet + MatrixShape>(fmt: &mut fmt::Formatter, a: &T) -> fmt::Result
{
	use std::cmp::max;
	
	let alt = fmt.flags & (1 << (fmt::rt::FlagAlternate as uint)) != 0;
	
	let w: &mut Writer = fmt;
	
	/* HACK: This could avoid allocating all the strings... */
	let col_widths: Vec<uint> =
		range(0, a.ncol())
		.map(|c|
			range(0, a.nrow())
			.map(|r| unsafe
				{
					a.raw_get(r, c)
				})
			.fold(0, |l, v| max(l, v.to_string().len())))
		.collect();
	
	for r in range(0, a.nrow())
	{
		let mut first = true;
		let mstr =
			if alt
			{
				", "
			}
			else
			{
				" "
			};
		let (lstr, rstr) =
			if alt
			{
				if a.nrow() == 1
				{
					("[", "]")
				}
				else if r == 0
				{
					("[", ";\n")
				}
				else if r == a.nrow() - 1
				{
					(" ", "]")
				}
				else
				{
					(" ", ";\n")
				}
			}
			else
			{
				if a.nrow() == 1
				{
					("[", "]")
				}
				else if r == 0
				{
					("⎡", "⎤\n")
				}
				else if r == a.nrow() - 1
				{
					("⎣", "⎦")
				}
				else
				{
					("⎢", "⎥\n")
				}
			};
		try!(write!(w, "{}", lstr).map_err(|_| fmt::WriteError))
		for c in range(0, a.ncol())
		{
			if !first
			{
				try!(write!(w, "{}", mstr).map_err(|_| fmt::WriteError))
			}
			first = false;
			unsafe
			{
				try!(write!(w, "{:>1$}", a.raw_get(r, c), *col_widths.get(c)).map_err(|_| fmt::WriteError))
			}
		}
		try!(write!(w, "{}", rstr).map_err(|_| fmt::WriteError))
	}
	Ok(())
}

impl
fmt::Show for
Matrix
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
