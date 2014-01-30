// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::cell::Cell;

use vector::elements::VectorElements;
use vector::slice::Slice;
use vector::Vector;

pub trait VectorGet
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32;
	fn get(&self, idx: uint) -> f32;
}

pub trait VectorSet
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32);
	fn set(&self, idx: uint, val: f32);
}

pub trait VectorAssign<T>
{
	fn assign(&self, v: T);
}

pub trait VectorSlice
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<Self>;
	fn slice(self, start: uint, end: uint) -> Slice<Self>;
}

pub trait VectorElems
{
	fn elems(self) -> VectorElements<Self>;
}

pub trait ToVector
{
	fn to_vec(self) -> Vector;
}

/* Hack necessary for operator overloading */
pub trait LengthEq
{
	fn len_eq(&self, other_len: uint) -> bool;
}

impl<LHS: VectorSet + Container,
     RHS: VectorGet + Container>
VectorAssign<RHS> for
LHS
{
	fn assign(&self, v: RHS)
	{
		assert!(self.len() == v.len());
		for i in range(0, v.len())
		{
			unsafe
			{
				self.unsafe_set(i, v.unsafe_get(i));
			}
		}
	}
}

impl<T: VectorGet + Container>
VectorElems for
T
{
	fn elems(self) -> VectorElements<T>
	{
		VectorElements::new(self)
	}
}

impl<T: VectorElems + VectorGet + Container>
ToVector for
T
{
	fn to_vec(self) -> Vector
	{
		Vector{ data: self.elems().map(|v| Cell::new(v)).collect() }
	}
}
