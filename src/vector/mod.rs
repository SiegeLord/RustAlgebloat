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
pub mod un_ops;
pub mod maps;
pub mod slice;
pub mod elements;
pub mod scalar;
#[cfg(test)]
mod test;

pub struct Vector
{
	priv data: ~[Cell<f32>]
}

impl Vector
{
	pub fn new(data: &[f32]) -> Vector
	{
		Vector{ data: data.map(|&v| Cell::new(v)) }
	}
}

impl
FromIterator<f32> for
Vector
{
	fn from_iterator<T: Iterator<f32>>(it: &mut T) -> Vector
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
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
&'l Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f32)
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
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f32)
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
	fn fmt(v: &Vector, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, &v)
	}
}

pub fn write_vec<T: VectorGet + Container>(w: &mut Writer, a: &T) -> fmt::Result
{
	let mut first = true;
	if_ok!(write!(w, "["))
	for i in range(0, a.len())
	{
		if !first
		{
			if_ok!(write!(w, " "))
		}
		first = false;
		unsafe
		{
			if_ok!(write!(w, "{}", a.unsafe_get(i)))
		}
	}
	write!(w, "]")
}
