// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use traits::{MatrixGet, MatrixElems, MatrixShape};

impl<T: MatrixGet<usize> + MatrixShape>
MatrixElems for
T
{
	fn elems(self) -> MatrixElements<T>
	{
		MatrixElements::new(self)
	}
}

#[derive(Copy, Clone)]
pub struct MatrixElements<T>
{
	base: T,
	idx: usize
}

impl<T: MatrixGet<usize> + MatrixShape>
MatrixElements<T>
{
	pub fn new(base: T) -> MatrixElements<T>
	{
		MatrixElements{ base: base, idx: 0 }
	}
}

impl<T: MatrixGet<usize> + MatrixShape>
Iterator for
MatrixElements<T>
{
	type Item = f64;
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
