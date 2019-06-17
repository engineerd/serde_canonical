#![cfg_attr(test, deny(warnings))]
#![warn(rust_2018_idioms)]

pub mod error;
pub mod ser;
pub mod canonical_value;

#[cfg(test)]
mod tests;
