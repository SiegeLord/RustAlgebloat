// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![crate_type = "lib"]
#![crate_name = "algebloat"]

#![feature(old_io)]
#![feature(rustc_private)]
#![feature(std_misc)]
#![feature(test)]
#![feature(rand)]
#![feature(core)]

#[macro_use]
extern crate algebloat_macros;

extern crate fmt_macros;

pub use matrix::*;
pub use traits::*;
pub use bin_ops::*;
pub use un_ops::*;
pub use un_funs::*;
pub use bin_funs::*;
pub use reductions::*;

pub mod matrix;
pub mod bin_funs;
pub mod traits;
pub mod transpose;
pub mod view;
pub mod row_accessor;
pub mod column_accessor;
pub mod elements;
pub mod hstack;
pub mod matrix_mul;
pub mod bin_ops;
pub mod scalar;
pub mod index;
pub mod un_ops;
pub mod un_funs;
pub mod vstack;
pub mod reductions;
pub mod reshape;
pub mod slice;
#[cfg(test)]
mod test;
