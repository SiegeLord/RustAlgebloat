use std::fmt;

use traits::{MatrixRawGet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixTranspose/*, MatrixFlat*/};
use transpose::Transposer;
use row_accessor::RowAccessor;
use column_accessor::ColumnAccessor;
use matrix::write_mat;

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
Container for
MatrixMul<LHS, RHS>
{
	#[inline]
	fn len(&self) -> uint
	{
		self.nrow() * self.ncol()
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

impl<LHS: MatrixShape,
     RHS: MatrixShape>
MatrixColumnAccess for
MatrixMul<LHS, RHS>
{
	unsafe fn unsafe_col(self, c: uint) -> ColumnAccessor<MatrixMul<LHS, RHS>>
	{
		ColumnAccessor::unsafe_new(self, c)
	}
	
	fn col(self, c: uint) -> ColumnAccessor<MatrixMul<LHS, RHS>>
	{
		ColumnAccessor::new(self, c)
	}
}

impl<LHS: MatrixShape,
     RHS: MatrixShape>
MatrixRowAccess for
MatrixMul<LHS, RHS>
{
	unsafe fn unsafe_row(self, r: uint) -> RowAccessor<MatrixMul<LHS, RHS>>
	{
		RowAccessor::unsafe_new(self, r)
	}
	
	fn row(self, r: uint) -> RowAccessor<MatrixMul<LHS, RHS>>
	{
		RowAccessor::new(self, r)
	}
}

impl<LHS,
     RHS>
MatrixTranspose for
MatrixMul<LHS, RHS>
{
	fn t(self) -> Transposer<MatrixMul<LHS, RHS>>
	{
		Transposer::new(self)
	}
}

impl<LHS: MatrixRawGet + MatrixShape,
     RHS: MatrixRawGet + MatrixShape>
fmt::Show for
MatrixMul<LHS, RHS>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
	}
}
