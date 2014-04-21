// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use vector::traits::VectorGet;

pub struct VectorElements<T>
{
	base: T,
	idx: uint
}

impl<T: VectorGet + Container>
VectorElements<T>
{
	pub fn new(base: T) -> VectorElements<T>
	{
		VectorElements{ base: base, idx: 0 }
	}
}

impl<T: VectorGet + Container>
Iterator<f64> for
VectorElements<T>
{
	fn next(&mut self) -> Option<f64>
	{
		let ret = if self.idx < self.base.len()
		{
			unsafe
			{
				Some(self.base.unsafe_get(self.idx))
			}
		}
		else
		{
			None
		};
		self.idx += 1;
		ret
	}
}
