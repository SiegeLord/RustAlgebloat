// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use matrix::transpose::Transposer;
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use matrix::flat_view::FlatView;
use matrix::view::View;
use safe_alias::SafeAlias;

pub trait MatrixGet
{
	unsafe fn unsafe_get(&self, r: uint, c: uint) -> f64;
	fn get(&self, r: uint, c: uint) -> f64;
}

pub trait MatrixSet
{
	unsafe fn unsafe_set(&self, r: uint, c: uint, val: f64);
	fn set(&self, r: uint, c: uint, val: f64);
}

pub trait MatrixShape
{
	fn ncol(&self) -> uint;
	fn nrow(&self) -> uint;
}

pub trait MatrixTranspose
{
	fn t(self) -> Transposer<Self>;
}

pub trait MatrixRowAccess
{
	unsafe fn unsafe_row(self, row: uint) -> RowAccessor<Self>;
	fn row(self, row: uint) -> RowAccessor<Self>;
}

pub trait MatrixColumnAccess
{
	unsafe fn unsafe_col(self, col: uint) -> ColumnAccessor<Self>;
	fn col(self, col: uint) -> ColumnAccessor<Self>;
}

pub trait MatrixFlat
{
	fn flat(self) -> FlatView<Self>;
}

pub trait MatrixView
{
	unsafe fn unsafe_view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
	fn view(self, row_start: uint, col_start: uint, row_end: uint, col_end: uint) -> View<Self>;
}

pub trait MatrixSafeAssign<RHS>
{
	fn assign(&self, m: RHS);
}

pub trait MatrixAliasAssign<RHS>
{
	fn alias_assign(&mut self, m: RHS);
	unsafe fn unsafe_assign(&self, m: RHS);
}

impl<LHS: MatrixShape + MatrixSet + SafeAlias,
     RHS: MatrixShape + MatrixGet + SafeAlias>
MatrixSafeAssign<RHS> for LHS
{
	fn assign(&self, m: RHS)
	{
		unsafe
		{
			self.unsafe_assign(m);
		}
	}
}

impl<LHS: MatrixShape + MatrixSet,
     RHS: MatrixShape + MatrixGet>
MatrixAliasAssign<RHS> for LHS
{
	unsafe fn unsafe_assign(&self, m: RHS)
	{
		assert_eq!(self.nrow(), m.nrow());
		assert_eq!(self.ncol(), m.ncol());
		for r in range(0, self.nrow())
		{
			for c in range(0, self.ncol())
			{
				self.unsafe_set(r, c, m.unsafe_get(r, c));
			}
		}
	}

	fn alias_assign(&mut self, m: RHS)
	{
		unsafe
		{
			(&*self).unsafe_assign(m);
		}
	}
}
