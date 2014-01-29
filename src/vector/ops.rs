// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::fmt;
use std::ops::{Add, Sub, Mul};

use matrix::traits::{MatrixShape, MatrixGet};
use matrix::row_accessor::RowAccessor;
use matrix::column_accessor::ColumnAccessor;
use vector::Vector;
use vector::write_vec;
use vector::traits::*;
use vector::slice::Slice;

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
			priv a: TA,
			priv b: TB,
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
				Slice::unsafe_new(self, start, end)
			}
	
			fn slice(self, start: uint, end: uint) -> Slice<$op_struct_name<TA, TB>>
			{
				Slice::new(self, start, end)
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
