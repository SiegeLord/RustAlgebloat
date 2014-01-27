// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::io::Writer;
use std::cell::Cell;

use matrix::{RowAccessor, ColumnAccessor, MatrixShape, MatrixGet};

pub trait VectorGet
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32;
	fn get(&self, idx: uint) -> f32;
}

pub trait VectorSet
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32);
	fn set(&self, idx: uint, val: f32);
}

pub trait VectorAssign<T>
{
	fn assign(&self, v: T);
}

pub trait VectorSlice
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<Self>;
	fn slice(self, start: uint, end: uint) -> Slice<Self>;
}

pub trait VectorElems
{
	fn elems(self) -> VectorElements<Self>;
}

impl<LHS: VectorSet + Container,
     RHS: VectorGet + Container>
VectorAssign<RHS> for
LHS
{
	fn assign(&self, v: RHS)
	{
		assert!(self.len() == v.len());
		for i in range(0, v.len())
		{
			unsafe
			{
				self.unsafe_set(i, v.unsafe_get(i));
			}
		}
	}
}

impl<T: VectorGet + Container>
VectorElems for
T
{
	fn elems(self) -> VectorElements<T>
	{
		VectorElements::new(self)
	}
}

pub struct Vector
{
	priv data: ~[Cell<f32>]
}

impl Vector
{
	pub fn new(data: &[f32]) -> Vector
	{
		Vector{ data: data.map(|&v| Cell::new(v)) }
	}
}

impl
FromIterator<f32> for
Vector
{
	fn from_iterator<T: Iterator<f32>>(it: &mut T) -> Vector
	{
		Vector{ data: it.by_ref().map(|v| Cell::new(v)).collect() }
	}
}

impl<'l>
VectorSlice for
&'l Vector
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		Slice{ base: self, start: start, end: end }
	}
	
	fn slice(self, start: uint, end: uint) -> Slice<&'l Vector>
	{
		assert!(start < end);
		assert!(end <= self.data.len());
		Slice{ base: self, start: start, end: end }
	}
}

impl<'l>
VectorGet for
&'l Vector
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
&'l Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f32)
	{
		self.data[idx].set(val);
	}
}

impl<'l>
Container for
&'l Vector
{
	fn len(&self) -> uint
	{
		self.data.len()
	}
}

impl<'l>
VectorGet for
Vector
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		(*self.data.unsafe_ref(idx)).get()
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx].get()
	}
}

impl<'l>
VectorSet for
Vector
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.data.unsafe_ref(idx).set(val);
	}

	fn set(&self, idx: uint, val: f32)
	{
		self.data[idx].set(val);
	}
}

impl<'l>
Container for
Vector
{
	fn len(&self) -> uint
	{
		self.data.len()
	}
}

impl
fmt::Default for
Vector
{
	fn fmt(v: &Vector, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, &v);
	}
}

pub struct VectorElements<T>
{
	priv base: T,
	priv idx: uint
}

impl<T: VectorGet + Container>
VectorElements<T>
{
	fn new(base: T) -> VectorElements<T>
	{
		VectorElements{ base: base, idx: 0 }
	}
}

impl<T: VectorGet + Container>
Iterator<f32> for
VectorElements<T>
{
	fn next(&mut self) -> Option<f32>
	{
		let ret = if self.idx < self.base.len()
		{
			unsafe
			{
				Some(self.base.unsafe_get(self.idx))
			}
		}
		else
		{
			None
		};
		self.idx += 1;
		ret
	}
}

pub struct Slice<T>
{
	priv base: T,
	priv start: uint,
	priv end: uint
}

impl<T: Clone>
Clone for
Slice<T>
{
	fn clone(&self) -> Slice<T>
	{
		Slice{ base: self.base.clone(), start: self.start, end: self.end }
	}
}

impl<T: VectorGet>
VectorGet for
Slice<T>
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32
	{
		self.base.unsafe_get(idx + self.start)
	}

	fn get(&self, idx: uint) -> f32
	{
		assert!(idx < self.len());
		unsafe
		{
			self.unsafe_get(idx)
		}
	}
}

impl<T: VectorSet>
VectorSet for
Slice<T>
{
	unsafe fn unsafe_set(&self, idx: uint, val: f32)
	{
		self.base.unsafe_set(idx + self.start, val);
	}
	
	fn set(&self, idx: uint, val: f32)
	{
		assert!(idx < self.len());
		unsafe
		{
			self.base.unsafe_set(idx + self.start, val);
		}
	}
}

impl<T>
VectorSlice for
Slice<T>
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<Slice<T>>
	{
		Slice{ base: self, start: start, end: end }
	}

	fn slice(self, start: uint, end: uint) -> Slice<Slice<T>>
	{
		assert!(start <= end);
		assert!(end <= self.len());
		Slice{ base: self, start: start, end: end }
	}
}

impl<T>
Container for
Slice<T>
{
	fn len(&self) -> uint
	{
		self.end - self.start
	}
}

impl<T: VectorGet>
fmt::Default for
Slice<T>
{
	fn fmt(v: &Slice<T>, buf: &mut fmt::Formatter)
	{
		write_vec(buf.buf, v);
	}
}

macro_rules! expr
{
	($e: expr) => { $e }
}

macro_rules! bin_op
{
	($op_name: ident, $op_method: ident, $op_struct_name: ident, $op: tt) =>
	{
		pub struct $op_struct_name<TA, TB>
		{
			a: TA,
			b: TB,
		}
		
		impl<TA: Container,
		     TB: Container>
		$op_struct_name<TA, TB>
		{
			pub unsafe fn unsafe_new(a: TA, b: TB) -> $op_struct_name<TA, TB>
			{
				$op_struct_name{ a: a, b: b }
			}
	
			pub fn new(a: TA, b: TB) -> $op_struct_name<TA, TB>
			{
				assert!(a.len() == b.len());
				$op_struct_name{ a: a, b: b }
			}
		}
		
		impl<'l,
		     TA: Clone + Container,
		     TB: Clone + Container>
		VectorSlice for
		$op_struct_name<TA, TB>
		{
			unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<$op_struct_name<TA, TB>>
			{
				Slice{ base: self, start: start, end: end }
			}
	
			fn slice(self, start: uint, end: uint) -> Slice<$op_struct_name<TA, TB>>
			{
				assert!(start <= end);
				assert!(end <= self.len());
				Slice{ base: self, start: start, end: end }
			}
		}
	
		impl<TA: Clone,
		     TB: Clone>
		Clone for
		$op_struct_name<TA, TB>
		{
			fn clone(&self) -> $op_struct_name<TA, TB>
			{
				$op_struct_name{ a: self.a.clone(), b: self.b.clone() }
			}
		}
	
		impl<'l,
		     TA: VectorGet + Container,
		     TB: VectorGet + Container>
		VectorGet for
		$op_struct_name<TA, TB>
		{
			unsafe fn unsafe_get(&self, idx: uint) -> f32
			{
				expr!(self.a.unsafe_get(idx) $op self.b.unsafe_get(idx))
			}
			
			fn get(&self, idx: uint) -> f32
			{
				assert!(idx < self.len());
				unsafe
				{
					expr!(self.a.unsafe_get(idx) $op self.b.unsafe_get(idx))
				}
			}
		}
	
		impl<'l,
		     TA: Container,
		     TB: Container>
		Container for $op_struct_name<TA, TB>
		{
			fn len(&self) -> uint
			{
				self.a.len()
			}
		}
		
		impl<'l,
		     RHS: VectorGet + Clone + Container>
		$op_name<RHS, $op_struct_name<&'l Vector, RHS>> for
		&'l Vector
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<&'l Vector, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}

		impl<RHS: VectorGet + Clone + Container,
		     T:   Clone>
		$op_name<RHS, $op_struct_name<Slice<T>, RHS>> for
		Slice<T>
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<Slice<T>, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}

		impl<RHS: VectorGet + Clone + Container,
		     T:   MatrixShape + Clone>
		$op_name<RHS, $op_struct_name<RowAccessor<T>, RHS>> for
		RowAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<RowAccessor<T>, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}

		impl<RHS: VectorGet + Clone + Container,
		     T:   MatrixShape + Clone>
		$op_name<RHS, $op_struct_name<ColumnAccessor<T>, RHS>> for
		ColumnAccessor<T>
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<ColumnAccessor<T>, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}
		
		impl<TA: VectorGet + Container,
		     TB: VectorGet + Container>
		fmt::Default for
		$op_struct_name<TA, TB>
		{
			fn fmt(v: &$op_struct_name<TA, TB>, buf: &mut fmt::Formatter)
			{
				write_vec(buf.buf, v);
			}
		}

	}
}

macro_rules! connect_op
{
	($op_struct_lhs: ident : $($op_name: ident, $op_method: ident, $op_struct_name: ident);+) =>
	{
		$(
			impl<RHS: VectorGet + Clone + Container,
				 TA:  VectorGet + Clone + Container,
				 TB:  VectorGet + Clone + Container>
			$op_name<RHS, $op_struct_name<$op_struct_lhs<TA, TB>, RHS>> for
			$op_struct_lhs<TA, TB>
			{
				fn $op_method(&self, rhs: &RHS) -> $op_struct_name<$op_struct_lhs<TA, TB>, RHS>
				{
					$op_struct_name::new(self.clone(), rhs.clone())
				}
			}
		)+
	}
}

macro_rules! connect_ops
{
	($($op_struct: ident),+) =>
	{
		$(
			connect_op!($op_struct:
			          Add, add, Adder;
			          Sub, sub, Subtracter;
			          Mul, mul, Multiplier;
			          Div, div, Divider)
		)+
	}
}

bin_op!(Add, add, Adder, +)
bin_op!(Sub, sub, Subtracter, -)
bin_op!(Mul, mul, Multiplier, *)
bin_op!(Div, div, Divider, /)

connect_ops!(Adder,
             Subtracter,
             Multiplier,
             Divider)

pub fn to_vec<T: VectorElems + VectorGet + Container>(a: T) -> Vector
{
	Vector{ data: a.elems().map(|v| Cell::new(v)).collect() }
}

pub fn write_vec<T: VectorGet + Container>(w: &mut Writer, a: &T)
{
	let mut first = true;
	write!(w, "[");
	for i in range(0, a.len())
	{
		if !first
		{
			write!(w, " ");
		}
		first = false;
		unsafe
		{
			write!(w, "{}", a.unsafe_get(i));
		}
	}
	write!(w, "]");
}

#[cfg(test)]
mod test
{
	extern mod extra;
	
	use super::*;
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
}
