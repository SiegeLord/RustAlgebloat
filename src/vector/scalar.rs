// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use vector::traits::{VectorGet, LengthEq};

impl
LengthEq for
f32
{
	fn len_eq(&self, _: uint) -> bool
	{
		true
	}
}

impl
VectorGet for
f32
{
	unsafe fn unsafe_get(&self, _: uint) -> f32
	{
		*self
	}
	
	fn get(&self, _: uint) -> f32
	{
		*self
	}
}
