// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use un_ops::{UnOp, MatrixUnOp};
use traits::MatrixShape;

macro_rules! un_fun
{
	($struct_name: ident, $trait_name: ident, $func_name: ident) =>
	{
		#[derive(Copy, Clone)]
		pub struct $struct_name;

		impl $struct_name
		{
			#[inline]
			pub fn new() -> $struct_name
			{
				$struct_name
			}
		}

		mod $func_name
		{
			#[allow(unused_imports)]
			use std::num::Float;

			#[inline]
			pub fn $func_name(a: f64) -> f64
			{
				a.$func_name()
			}
		}

		impl UnOp for $struct_name
		{
			fn op(&self, a: f64) -> f64
			{
				$func_name::$func_name(a)
			}
		}

		pub trait $trait_name
		{
			fn $func_name(self) -> MatrixUnOp<Self, $struct_name>;
		}

		impl<T: MatrixShape + Clone>
		$trait_name for
		T
		{
			fn $func_name(self) -> MatrixUnOp<T, $struct_name>
			{
				MatrixUnOp::new(self.clone(), $struct_name::new())
			}
		}
	}
}

un_fun!(AbsOp, MatrixAbsOp, abs);
un_fun!(AcosOp, MatrixAcosOp, acos);
un_fun!(AsinOp, MatrixAsinOp, asin);
un_fun!(AtanOp, MatrixAtanOp, atan);
un_fun!(CeilOp, MatrixCeilOp, ceil);
un_fun!(CosOp, MatrixCosOp, cos);
un_fun!(CoshOp, MatrixCoshOp, cosh);
un_fun!(ExpOp, MatrixExpOp, exp);
un_fun!(FloorOp, MatrixFloorOp, floor);
un_fun!(LnOp, MatrixLnOp, ln);
un_fun!(Log10Op, MatrixLog10Op, log10);
un_fun!(RoundOp, MatrixRoundOp, round);
un_fun!(SinOp, MatrixSinOp, sin);
un_fun!(SinhOp, MatrixSinhOp, sinh);
un_fun!(SqrtOp, MatrixSqrtOp, sqrt);
un_fun!(TanOp, MatrixTanOp, tan);
un_fun!(TanhOp, MatrixTanhOp, tanh);
