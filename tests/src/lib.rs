#![cfg_attr(asm_available, feature(asm))]

extern crate core;
#[cfg(test)]
#[macro_use]
extern crate uint;
extern crate ethereum_types;
#[cfg(test)]
#[macro_use]
extern crate crunchy;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
pub mod uint_tests;
