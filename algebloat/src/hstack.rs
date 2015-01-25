use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixHStack, SameShape};
use matrix::write_mat;

impl<L: MatrixShape,
     R: MatrixShape>
MatrixHStack<R> for
L
{
	unsafe fn unsafe_hstack(self, right: R) -> HStack<L, R>
	{
		HStack::unsafe_new(self, right)
	}

	fn hstack(self, right: R) -> HStack<L, R>
	{
		HStack::new(self, right)
	}
}

#[derive(Copy)]
pub struct HStack<L, R>
{
	left: L,
	right: R,
}

impl<L: MatrixShape,
     R: MatrixShape>
HStack<L, R>
{
	unsafe fn unsafe_new(left: L, right: R) -> HStack<L, R>
	{
		HStack{ left: left, right: right }
	}

	fn new(left: L, right: R) -> HStack<L, R>
	{
		assert_eq!(left.nrow(), right.nrow());
		HStack{ left: left, right: right }
	}
}

impl<L: MatrixRawGet + MatrixShape,
     R: MatrixRawGet>
MatrixRawGet for
HStack<L, R>
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		if c < self.left.ncol()
		{
			self.left.raw_get(r, c)
		}
		else
		{
			self.right.raw_get(r, c - self.left.ncol())
		}
	}
}

impl<L: MatrixRawSet + MatrixShape,
     R: MatrixRawSet>
MatrixRawSet for
HStack<L, R>
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		if c < self.left.ncol()
		{
			self.left.raw_set(r, c, val)
		}
		else
		{
			self.right.raw_set(r, c - self.left.ncol(), val)
		}
	}
}

impl<L: MatrixShape,
     R: MatrixShape>
MatrixShape for
HStack<L, R>
{
	fn nrow(&self) -> usize
	{
		self.left.nrow()
	}

	fn ncol(&self) -> usize
	{
		self.left.ncol() + self.right.ncol()
	}
}

impl<L: MatrixShape,
     R: MatrixShape>
SameShape for
HStack<L, R>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<L: Clone,
     R: Clone>
Clone for
HStack<L, R>
{
	fn clone(&self) -> HStack<L, R>
	{
		HStack{ left: self.left.clone(), right: self.right.clone() }
	}
}

impl<L: MatrixRawGet + MatrixShape,
     R: MatrixRawGet + MatrixShape>
fmt::Display for
HStack<L, R>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
