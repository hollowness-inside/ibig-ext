//! Module containing error handling types for Jacobi symbol calculations.
//!
//! This module provides:
//!
//! - The `Error` enum representing possible errors that can occur during the calculation.
//! - A `Result` type alias for representing the outcome of operations,
//!   wrapping either a successful value or an `Error`.

/// Error type used by the Jacobi symbol calculation.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Error indicating the modulo is even.
    /// The Jacobi symbol is only defined for odd positive integers.
    EvenModulo,

    /// Conversion error during internal calculations.
    /// This can occur when converting an `IBig` to a `UBig` type.
    ConversionError,
}

/// Alias for the `std::result::Result` type using the custom `Error` type.
/// Represents the possible outcomes of operations in this module.
pub type Result<T> = std::result::Result<T, Error>;
