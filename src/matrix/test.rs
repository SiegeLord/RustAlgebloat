// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use matrix::traits::{MatrixGet, MatrixSet, MatrixShape, MatrixRowAccess, MatrixColumnAccess,
                     MatrixView, MatrixTranspose, MatrixAssign, MatrixFlat};
use vector::traits::{VectorGet};

use super::*;

#[test]
fn flat_view()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0);
	let v = (&m).flat();
	assert_eq!(v.get(0), 1.0);
	assert_eq!(v.get(1), 2.0);
}

#[test]
fn assignment()
{
	let m1 = mat!(1.0, 2.0;
	              3.0, 4.0);
	let m2 = mat!(5.0, 6.0;
	              7.0, 8.0);
	let v1 = m1.view(0, 0, m1.nrow(), m1.ncol());
	m2.assign(&m1);
	assert_eq!(m2.get(0, 0), 1.0);
	m2.assign(v1);
	assert_eq!(m2.get(0, 0), 1.0);
}


#[test]
fn from_fn()
{
	let m = Matrix::from_fn(5, 5, |r, c| (r + c) as f64);
	assert_eq!(m.nrow(), 5);
	assert_eq!(m.ncol(), 5);
	assert_eq!(m.get(4, 4), 8.0);
}

#[test]
fn from_elem()
{
	let m = Matrix::from_elem(5, 5, 1.0);
	assert_eq!(m.nrow(), 5);
	assert_eq!(m.ncol(), 5);
	assert_eq!(m.get(0, 0), 1.0);
}

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
