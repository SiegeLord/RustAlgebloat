// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use vector::un_ops::{UnOp, VectorUnOp};

macro_rules! un_fun
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
			pub fn $func_name(a: f64) -> f64
			{
				a.$func_name()
			}
		}

		impl UnOp for $struct_name
		{
			fn op(&self, a: f64) -> f64
			{
				//~ f64::$func_name(a)
				//~ concat_idents!(dummy_, $func_name)::$func_name(a)
				$func_name::$func_name(a)
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

un_fun!(AbsOp, VectorAbsOp, abs)
un_fun!(AcosOp, VectorAcosOp, acos)
un_fun!(AsinOp, VectorAsinOp, asin)
un_fun!(AtanOp, VectorAtanOp, atan)
un_fun!(CeilOp, VectorCeilOp, ceil)
un_fun!(CosOp, VectorCosOp, cos)
un_fun!(CoshOp, VectorCoshOp, cosh)
un_fun!(ExpOp, VectorExpOp, exp)
un_fun!(FloorOp, VectorFloorOp, floor)
un_fun!(LnOp, VectorLnOp, ln)
un_fun!(Log10Op, VectorLog10Op, log10)
un_fun!(RoundOp, VectorRoundOp, round)
un_fun!(SinOp, VectorSinOp, sin)
un_fun!(SinhOp, VectorSinhOp, sinh)
un_fun!(SqrtOp, VectorSqrtOp, sqrt)
un_fun!(TanOp, VectorTanOp, tan)
un_fun!(TanhOp, VectorTanhOp, tanh)
