// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![feature(macro_rules)]

#![crate_type="lib"]
#![crate_name="algebloat_macros"]

//~ #[macro_export]
//~ macro_rules! vec
//~ {
	//~ ( $($e: expr),+) =>
	//~ {
		//~ Vector::new([$(
				//~ ($e) as f64,
		//~ )+])
	//~ }
//~ }

#[macro_export]
macro_rules! mat
{
	( $($($e: expr),+);+ ) =>
	{
		Matrix::new([$(
			&[$(
				($e) as f64,
			)+],
		)+])
	}
}

#[macro_export]
macro_rules! stack
{
    ($($a: expr),+; $($($b: expr),+);+) =>
    {
        (stack!($($a),+)).vstack(stack!($(stack!($($b),+));+))
    };
    ($a: expr, $($b: expr),+) =>
    {
        $a.hstack(stack!($($b),+))
    };
    ($e: expr) =>
    {
        $e
    }
}
