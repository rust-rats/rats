#![allow(incomplete_features)]
#![feature(generic_associated_types)]

#![feature(external_doc)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

#![doc(include = "../README.md")]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod core;
pub mod kernel;
