use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixVStack, SameShape};
use matrix::write_mat;

impl<T: MatrixShape,
     B: MatrixShape>
MatrixVStack<B> for
T
{
	unsafe fn unsafe_vstack(self, bot: B) -> VStack<T, B>
	{
		VStack::unsafe_new(self, bot)
	}

	fn vstack(self, bot: B) -> VStack<T, B>
	{
		VStack::new(self, bot)
	}
}

#[derive(Copy)]
pub struct VStack<T, B>
{
	top: T,
	bot: B,
}

impl<T: MatrixShape,
     B: MatrixShape>
VStack<T, B>
{
	unsafe fn unsafe_new(top: T, bot: B) -> VStack<T, B>
	{
		VStack{ top: top, bot: bot }
	}

	fn new(top: T, bot: B) -> VStack<T, B>
	{
		assert_eq!(top.ncol(), bot.ncol());
		VStack{ top: top, bot: bot }
	}
}

impl<T: MatrixRawGet + MatrixShape,
     B: MatrixRawGet>
MatrixRawGet for
VStack<T, B>
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		if r < self.top.nrow()
		{
			self.top.raw_get(r, c)
		}
		else
		{
			self.bot.raw_get(r - self.top.nrow(), c)
		}
	}
}

impl<T: MatrixRawSet + MatrixShape,
     B: MatrixRawSet>
MatrixRawSet for
VStack<T, B>
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		if r < self.top.nrow()
		{
			self.top.raw_set(r, c, val)
		}
		else
		{
			self.bot.raw_set(r - self.top.nrow(), c, val)
		}
	}
}

impl<T: MatrixShape,
     B: MatrixShape>
MatrixShape for
VStack<T, B>
{
	fn nrow(&self) -> usize
	{
		self.top.nrow() + self.bot.nrow()
	}

	fn ncol(&self) -> usize
	{
		self.top.ncol()
	}
}

impl<T: MatrixShape,
     B: MatrixShape>
SameShape for
VStack<T, B>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: Clone,
     B: Clone>
Clone for
VStack<T, B>
{
	fn clone(&self) -> VStack<T, B>
	{
		VStack{ top: self.top.clone(), bot: self.bot.clone() }
	}
}

impl<T: MatrixRawGet + MatrixShape,
     B: MatrixRawGet + MatrixShape>
fmt::Display for
VStack<T, B>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
