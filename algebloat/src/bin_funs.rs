// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use bin_ops::{BinOp, MatrixBinOp};
use traits::{SameShape, MatrixShape};

macro_rules! bin_fun
{
	($struct_name: ident, $trait_name: ident, $func_name: ident) =>
	{
		#[deriving(Copy, Clone)]
		pub struct $struct_name;

		impl $struct_name
		{
			pub fn new() -> $struct_name
			{
				$struct_name
			}
		}

		mod $func_name
		{
			#[allow(unused_imports)]
			use std::num::{Float, FloatMath};

			#[inline]
			pub fn $func_name(a: f64, b: f64) -> f64
			{
				a.$func_name(b)
			}
		}

		impl BinOp for $struct_name
		{
			fn op(&self, a: f64, b: f64) -> f64
			{
				$func_name::$func_name(a, b)
			}
		}

		pub trait $trait_name<RHS>
		{
			fn $func_name(self, rhs: RHS) -> MatrixBinOp<Self, RHS, $struct_name>;
		}

		impl<T: Clone + MatrixShape,
		     RHS: Clone + SameShape>
		$trait_name<RHS> for
		T
		{
			fn $func_name(self, rhs: RHS) -> MatrixBinOp<T, RHS, $struct_name>
			{
				MatrixBinOp::new(self.clone(), rhs.clone(), $struct_name::new())
			}
		}
	}
}

bin_fun!(PowOp, VectorPowOp, powf);
bin_fun!(HypotOp, VectorHypotOp, hypot);
bin_fun!(Atan2Op, VectorAtan2Op, atan2);
