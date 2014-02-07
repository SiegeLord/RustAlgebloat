// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;

use vector::write_vec;
use vector::traits::{VectorGet, VectorSet, VectorSlice, LengthEq};

pub struct Slice<T>
{
	priv base: T,
	priv start: uint,
	priv end: uint
}

impl<T: Container>
Slice<T>
{
	pub unsafe fn unsafe_new(base: T, start: uint, end: uint) -> Slice<T>
	{
		Slice{ base: base, start: start, end: end }
	}

	pub fn new(base: T, start: uint, end: uint) -> Slice<T>
	{
		assert!(start <= end);
		assert!(end <= base.len());
		Slice{ base: base, start: start, end: end }
	}
}

impl<T>
LengthEq for
Slice<T>
{
	fn len_eq(&self, other_len: uint) -> bool
	{
		other_len == self.len()
	}
}

impl<T: Clone>
Clone for
Slice<T>
{
	fn clone(&self) -> Slice<T>
	{
		Slice{ base: self.base.clone(), start: self.start, end: self.end }
	}
}

impl<T: VectorGet>
VectorGet for
Slice<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.base.unsafe_get(idx + self.start)
	}

	fn get(&self, idx: uint) -> f32
	{
		assert!(idx < self.len());
		unsafe
		{
			self.unsafe_get(idx)
		}
	}
}

impl<T: VectorSet>
VectorSet for
Slice<T>
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.base.unsafe_set(idx + self.start, val);
	}
	
	fn set(&self, idx: uint, val: f32)
	{
		assert!(idx < self.len());
		unsafe
		{
			self.base.unsafe_set(idx + self.start, val);
		}
	}
}

impl<T>
VectorSlice for
Slice<T>
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<Slice<T>>
	{
		Slice{ base: self, start: start, end: end }
	}

	fn slice(self, start: uint, end: uint) -> Slice<Slice<T>>
	{
		assert!(start <= end);
		assert!(end <= self.len());
		Slice{ base: self, start: start, end: end }
	}
}

impl<T>
Container for
Slice<T>
{
	fn len(&self) -> uint
	{
		self.end - self.start
	}
}

impl<T: VectorGet>
fmt::Show for
Slice<T>
{
	fn fmt(v: &Slice<T>, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_vec(buf.buf, v)
	}
}
