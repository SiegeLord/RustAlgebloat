// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use traits::{MatrixGet, MatrixShape};

pub trait MatrixReduce
{
	fn min(&self) -> Option<(usize, f64)>;
	fn max(&self) -> Option<(usize, f64)>;
}

impl<T: MatrixGet<usize> + MatrixShape>
MatrixReduce for
T
{
	fn min(&self) -> Option<(usize, f64)>
	{
		let l = self.len();
		if l == 0
		{
			None
		}
		else
		{
			let mut ret = self.get(0);
			let mut ret_i = 0;
			for i in 1..l
			{
				unsafe
				{
					let c = self.unsafe_get(i);
					if c < ret
					{
						ret = c;
						ret_i = i;
					}
				}
			}
			Some((ret_i, ret))
		}
	}

	fn max(&self) -> Option<(usize, f64)>
	{
		let l = self.len();
		if l == 0
		{
			None
		}
		else
		{
			let mut ret = self.get(0);
			let mut ret_i = 0;
			for i in 1..l
			{
				unsafe
				{
					let c = self.unsafe_get(i);
					if c > ret
					{
						ret = c;
						ret_i = i;
					}
				}
			}
			Some((ret_i, ret))
		}
	}
}

