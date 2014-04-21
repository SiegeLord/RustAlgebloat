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
	data: ~[Cell<f64>]
}

impl Vector
{
	pub fn new(data: &[f64]) -> Vector
	{
		Vector{ data: data.iter().map(|&v| Cell::new(v)).collect() }
	}

	pub fn from_elem(size: uint, elem: f64) -> Vector
	{
		use std::iter::Repeat;
		Vector{ data: Repeat::new(Cell::new(elem)).take(size).collect() }
	}
}

impl
FromIterator<f64> for
Vector
{
	fn from_iter<T: Iterator<f64>>(mut it: T) -> Vector
	{
		Vector{ data: it.by_ref().map(|v| Cell::new(v)).collect() }
	}
}

impl<'l>
VectorSlice for
&'l Vector
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		Slice::unsafe_new(self, start, end)
	}
	
	fn slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		Slice::new(self, start, end)
	}
}

impl<'l>
VectorGet for
&'l Vector
{
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f64
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
&'l Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f64)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f64)
	{
		self.data[idx].set(val);
	}
}

impl<'l>
Container for
&'l Vector
{
	fn len(&self) -> uint
	{
		self.data.len()
	}
}

impl<'l>
LengthEq for
&'l Vector
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<'l>
VectorGet for
Vector
{
	unsafe fn unsafe_get(&self, idx: uint) -> f64
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f64
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f64)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f64)
	{
		self.data[idx].set(val);
	}
}

impl<'l>
Container for
Vector
{
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