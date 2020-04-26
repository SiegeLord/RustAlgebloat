// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

extern crate test;

use super::*;
use chrono::prelude::*;

use self::test::Bencher;

fn rand_matrix() -> Matrix {
	Matrix::from_iter(1, 10, (0..10).map(|_| Utc::now().timestamp() as f64))
}

#[bench]
fn vec_speed_vec(bh: &mut Bencher) {
	let a = &rand_matrix();
	
	#[inline(never)]
	fn bug_14149(a: &Matrix)
	{
		for _ in 0..1000
		{
			a.assign((a + a * a) / a);
		}
		
		let mut sum = 0f64;
		for v in a.elems()
		{
			sum += v;
		}
		assert!(sum != 96.0);
	}

	bh.iter(|| {
		bug_14149(a);
	})
}

#[bench]
fn vec_speed_loop(bh: &mut Bencher) {
	let a = &rand_matrix();
	
	#[inline(never)]
	fn bug_14149(a: &Matrix)
	{
		for _ in 0..1000
		{
			for i in 0..a.len()
			{
				unsafe
				{
					let v = a.unsafe_get(i);
					a.unsafe_set(i, (v + v * v) / v)
				}
			}
		}
		
		let mut sum = 0f64;
		for v in a.elems()
		{
			sum += v;
		}
		assert!(sum != 96.0);
	}

	bh.iter(|| {
		bug_14149(a);
	})
}

#[test]
fn to_matrix()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0);
	let m2 = (&m).to_mat();
	assert_eq!(m.get((0, 0)), m2.get((0, 0)));
	assert_eq!(m.get((0, 2)), m2.get((0, 2)));
}

#[test]
fn bin_ops()
{
	let a = &mat!(1.0, 2.0, 3.0);
	let b = &mat!(2.0, 2.0, 2.0);
	
	let c = a * b + b;
	assert_eq!(c.get(0), 4.0);
	
	let c = (b - a) / b;
	assert_eq!(c.get(0), 0.5);
}

#[test]
fn matrix_multiply()
{
	let m = &mat!(1.0, 2.0, 3.0;
	              4.0, 5.0, 6.0);
	let m2 = m.mat_mul(m.t());
	assert_eq!(m2.nrow(), 2);
	assert_eq!(m2.ncol(), 2);
	assert_eq!(m2.get((0, 0)), 14.0);
	assert_eq!(m2.get((0, 1)), 32.0);
	assert_eq!(m2.get((1, 0)), 32.0);
	assert_eq!(m2.get((1, 1)), 77.0);
	m2.assign(m.mat_mul_lazy(m.t()));
	assert_eq!(m2.get((0, 0)), 14.0);
	assert_eq!(m2.get((0, 1)), 32.0);
	assert_eq!(m2.get((1, 0)), 32.0);
	assert_eq!(m2.get((1, 1)), 77.0);
}

#[test]
fn elems()
{
	let m = mat![1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0];
	let v1 = vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0];
	let v2: Vec<_> = m.elems().collect();
	assert_eq!(v1, v2);
}

#[test]
fn flat_view()
{
	let m = mat!(1.0, 2.0, 3.0;
	             4.0, 5.0, 6.0);
	assert_eq!(m.get(0), 1.0);
	assert_eq!(m.get(1), 2.0);
	assert_eq!(m.get(5), 6.0);
}

#[test]
fn assignment()
{
	let m1 = &mat!(1.0, 2.0;
	               3.0, 4.0);
	let m2 = &mat!(5.0, 6.0;
	               7.0, 8.0);
	let v1 = m1.view(0..m1.nrow(), 0..m1.ncol());
	m2.assign(m1);
	assert_eq!(m2.get((0, 0)), 1.0);
	m2.assign(v1);
	assert_eq!(m2.get((0, 0)), 1.0);
}


#[test]
fn from_fn()
{
	let m = Matrix::from_fn(5, 5, |r, c| (r + c) as f64);
	assert_eq!(m.nrow(), 5);
	assert_eq!(m.ncol(), 5);
	assert_eq!(m.get((4, 4)), 8.0);
}

#[test]
fn from_elem()
{
	let m = Matrix::from_elem(5, 5, 1.0);
	assert_eq!(m.nrow(), 5);
	assert_eq!(m.ncol(), 5);
	assert_eq!(m.get((0, 0)), 1.0);
}

#[test]
fn trans()
{
	let m = &mat!(1.0, 2.0, 3.0;
	              4.0, 5.0, 6.0);
	assert_eq!(m.nrow(), 2);
	assert_eq!(m.ncol(), 3);
	let t = m.t();
	assert_eq!(t.nrow(), 3);
	assert_eq!(t.ncol(), 2);
	assert_eq!(m.get((1, 2)), t.get((2, 1)));
}

#[test]
fn rows_and_cols()
{
	let m = &mat!(1.0, 2.0, 3.0;
	              4.0, 5.0, 6.0;
	              7.0, 8.0, 9.0);
	let v = m.row(0) + m.col(0).t();
	assert_eq!(v.get(1), 6.0);
}

#[test]
fn views()
{
	let m = &mat!(1.0, 2.0, 3.0;
	              4.0, 5.0, 6.0;
	              7.0, 8.0, 9.0);
	let m1 = m.view(..m.nrow() - 1, ..m.ncol() - 1);
	let m2 = m.view(1.., 1..);
	let v1 = m1.row(0) + m2.row(0);
	let v2 = m1.t().row(0) + m2.t().row(0);
	assert_eq!(v1.get(1), 8.0);
	assert_eq!(v2.get(1), 12.0);
}

#[test]
fn set()
{
	let m1 = &mat!(1.0, 2.0;
	               3.0, 4.0);
	let v1 = m1.view(0.., 0..);
	m1.set((0, 1), 5.0);
	v1.set((1, 0), 7.0);
	assert_eq!(m1.get((0, 1)), 5.0);
	assert_eq!(m1.get((1, 0)), 7.0);
	let t1 = m1.t();
	t1.set((1, 0), 11.0);
	assert_eq!(m1.get((0, 1)), 11.0);
}

#[test]
fn scalars()
{
	let a = &mat![1.0, 2.0, 3.0;
	              4.0, 5.0, 6.0];
	let b = a * 2.0f64;
	let c = b.view(..2, 1..2) * 3.0f64;
	assert_eq!(b.get((0, 0)), 2.0);
	assert_eq!(c.get((0, 0)), 12.0);
}

#[test]
fn neg()
{
	let a = &mat![1.0, 2.0, 3.0];
	let b = -(-a * 2.0f64);
	let c = -(b.view(..1, 1..) * 3.0f64);
	assert_eq!(b.get(0), 2.0);
	assert_eq!(c.get(0), -12.0);
}

#[test]
fn un_funs()
{
	use std::f64::consts::PI;
	let a = &mat![0.0, PI / 2.0, -PI / 2.0];
	let s1 = a.sin();
	let b = a.view(..1, 1..);
	let s2 = b.sin() + 1.0f64;
	assert_eq!(s1.get(1), 1.0);
	assert_eq!(s2.get(0), 2.0);
}

#[test]
fn bin_funs()
{
	let a = &mat![1.0, 2.0, 3.0];
	let b = &mat![1.0, 2.0, 3.0];
	let c = a.powf(b);
	assert_eq!(c.get(0), 1.0);
	assert_eq!(c.get(1), 4.0);
	assert_eq!(c.get(2), 27.0);
	let d = c + 1.0f64;
	assert_eq!(d.get(2), 28.0);
}

#[test]
fn min_max()
{
	let a = &mat![1.0, 2.0, 3.0, 7.0, -1.0, 5.0];
	let (min_idx, min) = a.min().unwrap();
	let (max_idx, max) = a.max().unwrap();
	assert_eq!(min_idx, 4);
	assert_eq!(min, -1.0);
	assert_eq!(max_idx, 3);
	assert_eq!(max, 7.0);
}

#[test]
fn slice()
{
	let m1 = &mat!(1.0, 2.0;
	               3.0, 4.0);
	let s = m1.slice(1..3) + 1.0f64;
	assert_eq!(s.nrow(), 2);
	assert_eq!(s.ncol(), 1);
	assert_eq!(s.get(0), 3.0);
	assert_eq!(s.get(1), 4.0);

	let s = m1.slice(1..);
	assert_eq!(s.len(), 3);
	assert_eq!(s.get(2), 4.0);

	let s = m1.slice(..2);
	assert_eq!(s.len(), 2);
	assert_eq!(s.get(1), 2.0);
}

#[test]
fn reshape()
{
	let m1 = &mat!(1.0, 2.0;
	               3.0, 4.0);
	let m2 = m1.reshape(4, 1) + 0.0f64;
	assert_eq!(m2.nrow(), 4);
	assert_eq!(m2.ncol(), 1);
	assert_eq!(m2.get(0), 1.0);
	assert_eq!(m2.get(1), 2.0);
	assert_eq!(m2.get(2), 3.0);
	let m3 = m2.reshape(2, 2);
	assert_eq!(m3.get((1, 1)), 4.0);
	let m4 = m1.reshape(4, 1);
	m4.set((2, 0), 5.0);
	assert_eq!(m1.get((1, 0)), 5.0);
}

#[test]
fn stack()
{
	let m1 = &mat!(1.0, 2.0;
	               3.0, 4.0);
	let m2 = &mat!(1.0, 2.0, 3.0;
	               3.0, 4.0, 5.0);
	let h = m1.hstack(m2);
	assert_eq!(h.ncol(), 5);
	assert_eq!(h.nrow(), 2);
	assert_eq!(h.get((1, 4)), 5.0);
	
	let h = m1.vstack(m2.t());
	assert_eq!(h.ncol(), 2);
	assert_eq!(h.nrow(), 5);
	assert_eq!(h.get((4, 1)), 5.0);
}

#[test]
fn convenience()
{
	let m1 = Matrix::eye(5);
	let m2 = Matrix::ones(5, 5);
	let m3 = Matrix::zeros(5, 5);

	assert_eq!(m1.size(), (5, 5));
	assert_eq!(m2.size(), (5, 5));
	assert_eq!(m3.size(), (5, 5));

	for (i, e) in m1.elems().enumerate()
	{
		assert_eq!(e, if i / 5 == i % 5 { 1.0 } else { 0.0 })
	}

	for e in m2.elems()
	{
		assert_eq!(e, 1.0)
	}

	for e in m3.elems()
	{
		assert_eq!(e, 0.0)
	}
}

#[test]
fn inv()
{
	let m1 = &Matrix::from_elem(2, 2, 2.0);
	let m2 = m1.inv();

	assert_eq!(m2.get((0, 0)), 0.5);
}
