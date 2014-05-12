// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use traits::MatrixGet;

pub struct MatrixElements<T>
{
	base: T,
	idx: uint
}

impl<T: MatrixGet<uint> + Container>
MatrixElements<T>
{
	pub fn new(base: T) -> MatrixElements<T>
	{
		MatrixElements{ base: base, idx: 0 }
	}
}

impl<T: MatrixGet<uint> + Container>
Iterator<f64> for
MatrixElements<T>
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
