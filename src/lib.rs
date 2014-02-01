// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#[feature(globs, macro_rules)];

#[crate_type="lib"];
#[crate_id="algebloat"];

pub use matrix::*;
pub use matrix::traits::*;
pub use vector::*;
pub use vector::bin_ops::*;
pub use vector::un_ops::*;
pub use vector::traits::*;
pub use vector::maps::*;

pub mod matrix;
pub mod vector;
