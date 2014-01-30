// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

extern mod extra;

use vector::*;
use vector::traits::*;
use self::extra::test::BenchHarness;
use std::rand::{weak_rng, Rng};

#[bench]
fn vec_speed_vec(bh: &mut BenchHarness) {
	let mut rng = weak_rng();
	
	let a = Vector::new(rng.gen_vec(10));

	bh.iter(|| {
		for _ in range(0, 100)
		{
			a.assign((&a + &a * &a) / &a);
		}
		
		let mut sum = 0f32;
		for v in (&a).elems()
		{
			sum += v;
		}
		assert!(sum != 96.0);
	})
}

#[bench]
fn vec_speed_loop(bh: &mut BenchHarness) {
	let mut rng = weak_rng();
	
	let a = Vector::new(rng.gen_vec(10));

	bh.iter(|| {
		for _ in range(0, 100)
		{
			for i in range(0, a.len())
			{
				unsafe
				{
					let v = a.unsafe_get(i);
					a.unsafe_set(i, (v + v * v) / v)
				}
			}
		}
		
		let mut sum = 0f32;
		for v in (&a).elems()
		{
			sum += v;
		}
		assert!(sum != 96.0);
	})
}

#[test]
fn vec_ops()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	let b = Vector::new([2.0, 2.0, 2.0]);
	
	let c = &a * &b + &b;
	assert_eq!(c.get(0), 4.0);
	
	let c = (&b - &a) / &b;
	assert_eq!(c.get(0), 0.5);
}

#[test]
fn vec_mut()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	a.set(0, 2.0);
	assert_eq!(a.get(0), 2.0);
	for _ in range(0, 5)
	{
		a.assign(&a + &a);
	}
	assert_eq!(a.get(2), 96.0);
}

#[test]
fn vec_slice()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	let b = Vector::new([2.0, 2.0, 2.0]);
	
	let a1 = a.slice(1, 3);
	let b1 = b.slice(0, 2);
	
	let c = a1 + b1;
	assert_eq!(c.get(0), 4.0);
	
	let c = a1.slice(1, 2);
	assert_eq!(c.get(0), 3.0);
}

#[test]
fn vec_mut_slice()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	let c = a.slice(1, 3);
	c.set(0, 10.0);
	assert_eq!(a.get(1), 10.0);
}

#[test]
fn vec_iter()
{
	let a: Vector = range(0, 5).map(|v| v as f32).collect();
	assert_eq!(a.elems().next().unwrap(), 0.0);
	
	//~ let mut b: Vector = range(0, 5).map(|v| v as f32).collect();
	//~ for v in (&mut b).iter()
	//~ {
		//~ *v = 1.0;
	//~ }
	//~ assert_eq!(b.get(0), 1.0);
}

#[test]
fn to_vec()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	let b = a.slice(1, a.len()).to_vec();
	b.set(0, 2.0);
	assert_eq!(a.get(0), 1.0);
	assert_eq!(b.get(0), 2.0);
}

#[test]
fn scalars()
{
	let a = Vector::new([1.0, 2.0, 3.0]);
	let b = &a * 2.0f32;
	let c = b.slice(1, 3) * 3.0f32;
	assert_eq!(b.get(0), 2.0);
	assert_eq!(c.get(0), 12.0);
}