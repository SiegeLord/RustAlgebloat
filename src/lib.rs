#[feature(globs, macro_rules)];

#[crate_type="lib"];
#[crate_id="algebloat"];

pub use matrix::*;
pub use vector::*;

pub mod matrix;
pub mod vector;
