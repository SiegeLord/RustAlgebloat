// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use vector::un_ops::{UnOp, VectorUnOp};
use std::f32;

macro_rules! map1
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

		impl UnOp for $struct_name
		{
			fn op(&self, a: f32) -> f32
			{
				f32::$func_name(a)
			}
		}

		pub trait $trait_name
		{
			fn $func_name(self) -> VectorUnOp<Self, $struct_name>;
		}

		impl<T: Clone>
		$trait_name for
		T
		{
			fn $func_name(self) -> VectorUnOp<T, $struct_name>
			{
				VectorUnOp::new(self.clone(), $struct_name::new())
			}
		}
	}
}

map1!(AbsOp, VectorAbsOp, abs)
map1!(AcosOp, VectorAcosOp, acos)
map1!(AsinOp, VectorAsinOp, asin)
map1!(AtanOp, VectorAtanOp, atan)
map1!(CeilOp, VectorCeilOp, ceil)
map1!(CosOp, VectorCosOp, cos)
map1!(CoshOp, VectorCoshOp, cosh)
map1!(ExpOp, VectorExpOp, exp)
map1!(FloorOp, VectorFloorOp, floor)
map1!(LnOp, VectorLnOp, ln)
map1!(Log10Op, VectorLog10Op, log10)
map1!(RoundOp, VectorRoundOp, round)
map1!(SinOp, VectorSinOp, sin)
map1!(SinhOp, VectorSinhOp, sinh)
map1!(SqrtOp, VectorSqrtOp, sqrt)
map1!(TanOp, VectorTanOp, tan)
map1!(TanhOp, VectorTanhOp, tanh)
