// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![feature(globs, macro_rules, phase)]

#![crate_type="lib"]
#![crate_id="algebloat"]
#![license = "LGPL3"]

#[phase(syntax)]
extern crate algebloat_macros;

pub use matrix::*;
pub use traits::*;
pub use bin_ops::*;

//~ pub use vector::*;
//~ pub use vector::bin_ops::*;
//~ pub use vector::bin_funs::*;
//~ pub use vector::un_ops::*;
//~ pub use vector::un_funs::*;
//~ pub use vector::reductions::*;
//~ pub use vector::traits::*;

pub mod matrix;
pub mod traits;
pub mod transpose;
pub mod view;
pub mod row_accessor;
pub mod column_accessor;
pub mod elements;
pub mod matrix_mul;
pub mod bin_ops;
pub mod index;
#[cfg(test)]
mod test;
