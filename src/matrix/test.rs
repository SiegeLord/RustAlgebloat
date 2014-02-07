// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

extern mod extra;

use matrix::traits::{MatrixGet, MatrixShape, MatrixRowAccess, MatrixColumnAccess, MatrixView, MatrixTranspose};
use vector::traits::{VectorGet};

use super::*;
//~ use self::extra::test::BenchHarness;
//~ use std::rand::{weak_rng, Rng};

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
