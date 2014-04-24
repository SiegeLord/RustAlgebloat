// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use vector::bin_ops::{BinOp, VectorBinOp};
use vector::traits::LengthEq;

macro_rules! bin_fun
{
	($struct_name: ident, $trait_name: ident, $func_name: ident) =>
	{
		#[deriving(Clone)]
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
			fn $func_name(self, rhs: RHS) -> VectorBinOp<Self, RHS, $struct_name>;
		}

		impl<T: Clone + Container,
		     RHS: Clone + LengthEq>
		$trait_name<RHS> for
		T
		{
			fn $func_name(self, rhs: RHS) -> VectorBinOp<T, RHS, $struct_name>
			{
				VectorBinOp::new(self.clone(), rhs.clone(), $struct_name::new())
			}
		}
	}
}

bin_fun!(PowOp, VectorPowOp, powf)
bin_fun!(HypotOp, VectorHypotOp, hypot)
bin_fun!(Atan2Op, VectorAtan2Op, atan2)
