// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use matrix::traits::{MatrixGet, MatrixSet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixView, MatrixTranspose, MatrixSafeAssign, MatrixAliasAssign};
use vector::traits::{VectorGet};

use super::*;

#[test]
fn trans()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0);
	assert_eq!(m.nrow(), 2);
	assert_eq!(m.ncol(), 3);
	let t = m.t();
	assert_eq!(t.nrow(), 3);
	assert_eq!(t.ncol(), 2);
	assert_eq!(m.get(1, 2), t.get(2, 1));
}

#[test]
fn rows_and_cols()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0;
	             7.0, 8.0, 9.0);
	let v = m.row(0) + m.col(0);
	assert_eq!(v.get(1), 6.0);
}

#[test]
fn views()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0;
	             7.0, 8.0, 9.0);
	let m1 = m.view(0, 0, m.nrow() - 1, m.ncol() - 1);
	let m2 = m.view(1, 1, m.nrow(), m.ncol());
	let v1 = m1.row(0) + m2.row(0);
	let v2 = m1.t().row(0) + m2.t().row(0);
	assert_eq!(v1.get(1), 8.0);
	assert_eq!(v2.get(1), 12.0);
}

#[test]
fn set()
{
	let m1 = mat!(1.0, 2.0;
	              3.0, 4.0);
	let v1 = m1.view(0, 0, m1.nrow(), m1.ncol());
	m1.set(0, 1, 5.0);
	v1.set(1, 0, 7.0);
	assert_eq!(m1.get(0, 1), 5.0);
	assert_eq!(m1.get(1, 0), 7.0);
	let t1 = m1.t();
	t1.set(1, 0, 11.0);
	assert_eq!(m1.get(0, 1), 11.0);
}

#[test]
fn aliasing()
{
	let m1 = mat!(1.0, 2.0;
	              3.0, 4.0);
	let mut m2 = mat!(5.0, 6.0;
	                  7.0, 8.0);
	let m3 = mat!(5.0, 6.0;
	              7.0, 8.0);
	let v1 = m1.view(0, 0, m1.nrow(), m1.ncol());
	m2.assign(&m1);
	assert_eq!(m2.get(0, 0), 1.0);
	m2.assign(v1);
	assert_eq!(m2.get(0, 0), 1.0);
	let mut t1 = m1.t();
	m2.alias_assign(t1);
	assert_eq!(m2.get(1, 0), 2.0);
	unsafe
	{
		m3.unsafe_assign(t1);
	}
	assert_eq!(m3.get(1, 0), 2.0);
	t1.alias_assign(&m3);
	assert_eq!(m1.get(0, 1), 2.0);
	
	let m1 = mat!(1.0, 2.0;
	              3.0, 4.0);
	let mut m2 = mat!(0.0, 0.0;
	                  0.0, 0.0);
	for _ in range(0, 2)
	{
		m2.alias_assign(m1.t());
		m2.set(0, 0, 0.0);
		m2.assign(&m2);
	}
	assert_eq!(m2.get(0, 0), 0.0);
	assert_eq!(m2.get(1, 1), 4.0);
}
