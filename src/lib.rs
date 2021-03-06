#![allow(incomplete_features)]
#![feature(generic_associated_types)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod core;
pub mod kernel;
