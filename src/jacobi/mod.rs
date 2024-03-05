//! Module for calculating the Jacobi symbol.
//!
//! The Jacobi symbol is a generalization of the Legendre symbol to integers that are not necessarily prime.
//! It is denoted as (a|n), where a is any integer and n is an odd positive integer.
//!
//! This module provides:
//!
//! - The `JacobiValue` enum representing possible values of the Jacobi symbol.
//! - The `Jacobi` trait for calculating the Jacobi symbol over different integer types.
//! - Implementations of the `Jacobi` trait for `UBig` and `IBig` types from the `ibig` crate.

pub mod result;

mod jacobi;
pub use jacobi::{Jacobi, JacobiValue};

#[cfg(test)]
mod tests;
