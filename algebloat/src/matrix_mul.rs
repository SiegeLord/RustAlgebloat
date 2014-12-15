use std::fmt;

use traits::{MatrixRawGet, MatrixShape, MatrixMultiply, ToMatrix};
use matrix::{Matrix, write_mat};

impl<LHS: MatrixShape + MatrixRawGet,
     RHS: MatrixShape + MatrixRawGet>
MatrixMultiply<RHS> for LHS
{
	unsafe fn unsafe_mat_mul(self, rhs: RHS) -> Matrix
	{
		MatrixMul::unsafe_new(self, rhs).to_mat()
	}

	unsafe fn unsafe_mat_mul_lazy(self, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		MatrixMul::unsafe_new(self, rhs)
	}

	fn mat_mul(self, rhs: RHS) -> Matrix
	{
		MatrixMul::new(self, rhs).to_mat()
	}

	fn mat_mul_lazy(self, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		MatrixMul::new(self, rhs)
	}
}

#[deriving(Copy)]
pub struct MatrixMul<LHS, RHS>
{
	lhs: LHS,
	rhs: RHS,
}

impl<LHS: MatrixShape,
     RHS: MatrixShape>
MatrixMul<LHS, RHS>
{
	pub unsafe fn unsafe_new(lhs: LHS, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		MatrixMul{ lhs: lhs, rhs: rhs }
	}

	pub fn new(lhs: LHS, rhs: RHS) -> MatrixMul<LHS, RHS>
	{
		assert_eq!(lhs.ncol(), rhs.nrow());
		MatrixMul{ lhs: lhs, rhs: rhs }
	}
}

impl<LHS: MatrixRawGet + MatrixShape,
     RHS: MatrixRawGet + MatrixShape>
MatrixRawGet for
MatrixMul<LHS, RHS>
{
	unsafe fn raw_get(&self, r: uint, c: uint) -> f64
	{
		let mut ret = 0.0;
		
		for z in range(0, self.lhs.ncol())
		{
			ret += self.lhs.raw_get(r, z) * self.rhs.raw_get(z, c);
		}
		ret
	}
}

impl<LHS: MatrixShape,
     RHS: MatrixShape>
MatrixShape for
MatrixMul<LHS, RHS>
{
	fn nrow(&self) -> uint
	{
		self.lhs.nrow()
	}
	fn ncol(&self) -> uint
	{
		self.rhs.ncol()
	}
}

impl<LHS: Clone,
     RHS: Clone>
Clone for
MatrixMul<LHS, RHS>
{
	fn clone(&self) -> MatrixMul<LHS, RHS>
	{
		MatrixMul{ rhs: self.rhs.clone(), lhs: self.lhs.clone() }
	}
}

impl<LHS: MatrixRawGet + MatrixShape,
     RHS: MatrixRawGet + MatrixShape>
fmt::Show for
MatrixMul<LHS, RHS>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
