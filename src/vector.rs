use std::ops::{Add, Sub, Mul};
use std::vec;
use std::fmt;
use std::io::Writer;

pub trait VectorGet
{
	unsafe fn unsafe_get(&self, idx: uint) -> f32;
	fn get(&self, idx: uint) -> f32;
}

pub trait VectorSet
{
	unsafe fn unsafe_set(&mut self, idx: uint, val: f32);
	fn set(&mut self, idx: uint, val: f32);
}

pub trait VectorLen
{
	fn len(&self) -> uint;
}

pub trait VectorSlice
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<Self>;
	fn slice(self, start: uint, end: uint) -> Slice<Self>;
}

pub struct Vector
{
	priv data: ~[f32]
}

impl Vector
{
	pub fn new(data: &[f32]) -> Vector
	{
		Vector{ data: data.to_owned() }
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
VectorSlice for
&'l mut Vector
{
	unsafe fn unsafe_slice(self, start: uint, end: uint) -> Slice<&'l mut Vector>
	{
		Slice{ base: self, start: start, end: end }
	}
	
	fn slice(self, start: uint, end: uint) -> Slice<&'l mut Vector>
	{
		assert!(start <= end);
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
		*self.data.unsafe_ref(idx)
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx]
	}
}

impl<'l>
VectorSet for
&'l mut Vector
{
	unsafe fn unsafe_set(&mut self, idx: uint, val: f32)
	{
		self.data.unsafe_set(idx, val);
	}

	fn set(&mut self, idx: uint, val: f32)
	{
		self.data[idx] = val;
	}
}

impl<'l>
VectorLen for
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
		*self.data.unsafe_ref(idx)
	}
	
	fn get(&self, idx: uint) -> f32
	{
		self.data[idx]
	}
}

impl<'l>
VectorSet for
Vector
{
	unsafe fn unsafe_set(&mut self, idx: uint, val: f32)
	{
		self.data.unsafe_set(idx, val);
	}

	fn set(&mut self, idx: uint, val: f32)
	{
		self.data[idx] = val;
	}
}

impl<'l>
VectorLen for
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

pub struct Slice<T>
{
	base: T,
	start: uint,
	end: uint
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
	unsafe fn unsafe_set(&mut self, idx: uint, val: f32)
	{
		self.base.unsafe_set(idx + self.start, val);
	}
	
	fn set(&mut self, idx: uint, val: f32)
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
VectorLen for
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
		
		impl<TA: VectorLen,
		     TB: VectorLen>
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
		     TA: Clone + VectorLen,
		     TB: Clone + VectorLen>
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
		     TA: VectorGet + VectorLen,
		     TB: VectorGet + VectorLen>
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
		     TA: VectorLen,
		     TB: VectorLen>
		VectorLen for $op_struct_name<TA, TB>
		{
			fn len(&self) -> uint
			{
				self.a.len()
			}
		}
		
		impl<'l,
		     RHS: VectorGet + Clone + VectorLen>
		$op_name<RHS, $op_struct_name<&'l Vector, RHS>> for
		&'l Vector
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<&'l Vector, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}

		impl<RHS: VectorGet + Clone + VectorLen,
		     T:   Clone>
		$op_name<RHS, $op_struct_name<Slice<T>, RHS>> for
		Slice<T>
		{
			fn $op_method(&self, rhs: &RHS) -> $op_struct_name<Slice<T>, RHS>
			{
				$op_struct_name::new(self.clone(), rhs.clone())
			}
		}
		
		impl<TA: VectorGet + VectorLen,
		     TB: VectorGet + VectorLen>
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
			impl<RHS: VectorGet + Clone + VectorLen,
				 TA:  VectorGet + Clone + VectorLen,
				 TB:  VectorGet + Clone + VectorLen>
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

pub fn to_vec<T: VectorGet + VectorLen>(a: T) -> Vector
{
	let mut ret = Vector{ data: vec::with_capacity(a.len()) };
	for i in range(0, a.len())
	{
		ret.data.push(a.get(i))
	}
	ret
}

pub fn write_vec<T: VectorGet + VectorLen>(w: &mut Writer, a: &T)
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
	let mut a = Vector::new([1.0, 2.0, 3.0]);
	a.set(0, 2.0);
	assert_eq!(a.get(0), 2.0);
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
	let mut a = Vector::new([1.0, 2.0, 3.0]);
	{
		let mut c = (&mut a).slice(1, 3);
		c.set(0, 10.0);
	}
	assert_eq!(a.get(1), 10.0);
}
