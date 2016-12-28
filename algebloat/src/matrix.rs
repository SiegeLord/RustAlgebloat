// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::vec::Vec;
use std::fmt;
use std::cell::Cell;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, SameShape};

pub struct Matrix
{
	data: Vec<Cell<f64>>,
	nrow: usize,
	ncol: usize
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

	pub fn eye(size: usize) -> Matrix
	{
		Matrix::from_fn(size, size, |r, c| if r == c { 1.0 } else { 0.0 })
	}

	pub fn zeros(nrow: usize, ncol: usize) -> Matrix
	{
		Matrix::from_elem(nrow, ncol, 0.0)
	}

	pub fn ones(nrow: usize, ncol: usize) -> Matrix
	{
		Matrix::from_elem(nrow, ncol, 1.0)
	}

	pub fn from_elem(nrow: usize, ncol: usize, elem: f64) -> Matrix
	{
		let mut mat_data = Vec::with_capacity(nrow * ncol);
		for _ in 0..nrow * ncol {
			mat_data.push(Cell::new(elem));
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}

	pub fn from_fn<F: FnMut(usize, usize) -> f64>(nrow: usize, ncol: usize, mut cb: F) -> Matrix
	{
		let mut mat_data = Vec::with_capacity(nrow * ncol);
		for r in 0..nrow
		{
			for c in 0..ncol
			{
				mat_data.push(Cell::new(cb(r, c)));
			}
		}
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}

	pub fn from_iter<T: Iterator<Item=f64>>(nrow: usize, ncol: usize, t: T) -> Matrix
	{
		let mat_data: Vec<Cell<f64>> = t.map(|v| Cell::new(v)).collect();
		assert_eq!(mat_data.len(), nrow * ncol);
		Matrix{ data: mat_data, nrow: nrow, ncol: ncol }
	}
}

impl<'l>
MatrixRawGet for
&'l Matrix
{
	#[inline]
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.data[..].get_unchecked(c + r * self.ncol).get()
	}
}

impl<'l>
MatrixRawSet for
&'l Matrix
{
	#[inline]
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		self.data[..].get_unchecked(c + r * self.ncol).set(val);
	}
}

impl<'l>
MatrixShape for
&'l Matrix
{
	#[inline]
	fn nrow(&self) -> usize
	{
		self.nrow
	}

	#[inline]
	fn ncol(&self) -> usize
	{
		self.ncol
	}
}

impl<'l>
SameShape for
&'l Matrix
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl
MatrixRawGet for
Matrix
{
	#[inline]
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.data[..].get_unchecked(c + r * self.ncol).get()
	}
}

impl
MatrixRawSet for
Matrix
{
	#[inline]
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		self.data[..].get_unchecked(c + r * self.ncol).set(val);
	}
}

impl
MatrixShape for
Matrix
{
	#[inline]
	fn nrow(&self) -> usize
	{
		self.nrow
	}

	#[inline]
	fn ncol(&self) -> usize
	{
		self.ncol
	}
}

pub fn write_mat<T: MatrixRawGet + MatrixShape>(w: &mut fmt::Formatter, a: &T) -> fmt::Result
{
	use std::cmp::max;
	
	/* HACK: This could avoid allocating all the strings... */
	let col_widths: Vec<usize> =
		(0..a.ncol())
		.map(|c|
			(0..a.nrow())
			.map(|r| unsafe
				{
					a.raw_get(r, c)
				})
			.fold(0, |l, v| max(l, v.to_string().len())))
		.collect();
	
	for r in 0..a.nrow()
	{
		let mut first = true;
		let mstr =
			if w.alternate()
			{
				", "
			}
			else
			{
				" "
			};
		let (lstr, rstr) =
			if w.alternate()
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
		try!(write!(w, "{}", lstr).map_err(|_| fmt::Error));
		for c in 0..a.ncol()
		{
			if !first
			{
				try!(write!(w, "{}", mstr).map_err(|_| fmt::Error));
			}
			first = false;
			unsafe
			{
				try!(write!(w, "{:>1$}", a.raw_get(r, c), col_widths[c]).map_err(|_| fmt::Error));
			}
		}
		try!(write!(w, "{}", rstr).map_err(|_| fmt::Error));
	}
	Ok(())
}

impl
fmt::Display for
Matrix
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
