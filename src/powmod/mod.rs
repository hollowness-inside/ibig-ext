//! Example
//!
//! ```rust
//! use ibig::ubig;
//! use powmod::PowMod;
//!
//! fn main() {
//!     let base = ubig!(987654321);
//!     let exponent = ubig!(99999);
//!     let modulo = ubig!(123);
//!
//!     let remainder = base.powmod(exponent, &modulo);
//!
//!     println!("The remainder is {remainder}");
//! }
//! ```

mod powmod;
pub use powmod::PowMod;

#[cfg(test)]
mod tests;
