#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

pub mod canonical_value;
pub mod error;
pub mod ser;

#[cfg(test)]
mod tests;
