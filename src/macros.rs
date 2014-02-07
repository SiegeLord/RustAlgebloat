// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#[macro_export]
macro_rules! vec
{
	( $($e: expr),+) =>
	{
		Vector::new([$(
				($e) as f32,
		)+])
	}
}

#[macro_export]
macro_rules! mat
{
	( $($($e: expr),+);+ ) =>
	{
		Matrix::new([$(
			&[$(
				($e) as f32,
			)+],
		)+])
	}
}
