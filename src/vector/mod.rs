// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::io::Writer;
use std::cell::Cell;

use vector::traits::{VectorGet, VectorSet, VectorSlice, LengthEq};
//~ use vector::ops::;
use vector::slice::Slice;

pub mod traits;
pub mod bin_ops;
pub mod bin_funs;
pub mod un_ops;
pub mod un_funs;
pub mod slice;
pub mod elements;
pub mod scalar;
pub mod reductions;
#[cfg(test)]
mod test;

pub struct Vector
{
	data: Vec<Cell<f64>>
}

impl Vector
{
	#[inline]
	pub fn new(data: &[f64]) -> Vector
	{
		Vector{ data: data.iter().map(|&v| Cell::new(v)).collect() }
	}

	#[inline]
	pub fn from_elem(size: uint, elem: f64) -> Vector
	{
		use std::iter::Repeat;
		Vector{ data: Repeat::new(Cell::new(elem)).take(size).collect() }
	}

	#[inline]
	pub fn from_fn(size: uint, cb: |uint| -> f64) -> Vector
	{
		Vector{ data: range(0, size).map(|i| Cell::new(cb(i))).collect() }
	}
}

impl
FromIterator<f64> for
Vector
{
	#[inline]
	fn from_iter<T: Iterator<f64>>(mut it: T) -> Vector
	{
		Vector{ data: it.by_ref().map(|v| Cell::new(v)).collect() }
	}
}

impl<'l>
VectorSlice for
&'l Vector
{
	#[inline]
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		Slice::unsafe_new(self, start, end)
	}

	#[inline]
	fn slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		Slice::new(self, start, end)
	}
}

impl<'l>
VectorGet for
&'l Vector
{
	#[inline]
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		(*self.data.as_slice().unsafe_ref(idx)).get()
	}

	#[inline]
	fn get(&self, idx: uint) -> f64
	{
		self.data.get(idx).get()
	}
}

impl<'l>
VectorSet for
&'l Vector
{
	#[inline]
	unsafe fn unsafe_set(&self, idx: uint, val: f64)
	{
		self.data.as_slice().unsafe_ref(idx).set(val);
	}

	#[inline]
	fn set(&self, idx: uint, val: f64)
	{
		self.data.get(idx).set(val);
	}
}

impl<'l>
Container for
&'l Vector
{
	#[inline]
	fn len(&self) -> uint
	{
		self.data.len()
	}
}

impl<'l>
LengthEq for
&'l Vector
{
	#[inline]
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<'l>
VectorGet for
Vector
{
	#[inline]
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		(*self.data.as_slice().unsafe_ref(idx)).get()
	}
	
	#[inline]
	fn get(&self, idx: uint) -> f64
	{
		self.data.get(idx).get()
	}
}

impl<'l>
VectorSet for
Vector
{
	#[inline]
	unsafe fn unsafe_set(&self, idx: uint, val: f64)
	{
		self.data.as_slice().unsafe_ref(idx).set(val);
	}

	#[inline]
	fn set(&self, idx: uint, val: f64)
	{
		self.data.get(idx).set(val);
	}
}

impl<'l>
Container for
Vector
{
	#[inline]
	fn len(&self) -> uint
	{
		self.data.len()
	}
}

impl
fmt::Show for
Vector
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, self)
	}
}

pub fn write_vec<T: VectorGet + Container>(w: &mut Writer, a: &T) -> fmt::Result
{
	let mut first = true;
	try!(write!(w, "["))
	for i in range(0, a.len())
	{
		if !first
		{
			try!(write!(w, " "))
		}
		first = false;
		unsafe
		{
			try!(write!(w, "{}", a.unsafe_get(i)))
		}
	}
	write!(w, "]")
}
