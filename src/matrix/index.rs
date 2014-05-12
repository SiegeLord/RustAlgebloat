// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use matrix::traits::{MatrixShape, MatrixRawSet, MatrixRawGet};

pub trait MatrixIndexGet<T>
{
	fn unsafe_get_idx(&self, mat: &T) -> f64;
	fn get_idx(&self, mat: &T) -> f64;
}

pub trait MatrixIndexSet<T>
{
	fn unsafe_set_idx(&self, mat: &T, v: f64);
	fn set_idx(&self, mat: &T, v: f64);
}

macro_rules! index_impl
{
	($idx_type: ty, $rc_expr: expr) =>
	{
		impl<T: MatrixRawGet + MatrixShape>
		     MatrixIndexGet<T> for
		$idx_type
		{
			fn unsafe_get_idx(&self, mat: &T) -> f64
			{
				let (r, c) = $rc_expr;
				unsafe
				{
					mat.raw_get(r, c)
				}
			}

			fn get_idx(&self, mat: &T) -> f64
			{
				let (r, c) = $rc_expr;
				assert!(r < mat.nrow());
				assert!(c < mat.ncol());
				unsafe
				{
					mat.raw_get(r, c)
				}
			}
		}

		impl<T: MatrixRawSet + MatrixShape>
		     MatrixIndexSet<T> for
		$idx_type
		{
			fn unsafe_set_idx(&self, mat: &T, v: f64)
			{
				let (r, c) = $rc_expr;
				unsafe
				{
					mat.raw_set(r, c, v)
				}
			}

			fn set_idx(&self, mat: &T, v: f64)
			{
				let (r, c) = $rc_expr;
				assert!(r < mat.nrow());
				assert!(c < mat.ncol());
				unsafe
				{
					mat.raw_set(r, c, v)
				}
			}
		}
	}
}

fn to_rc(idx: uint, ncol: uint) -> (uint, uint)
{
	(idx / ncol, idx % ncol)
}

index_impl!((uint, uint), {*self})
index_impl!((int, int), {let (r, c) = *self; (r as uint, c as uint)})
index_impl!(uint, {to_rc(*self, mat.ncol())})
index_impl!(int,  {to_rc(*self as uint, mat.ncol())})
