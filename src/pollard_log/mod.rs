//! This library provides the `discrete_log` function for calculating discrete logarithms using the
//! Pollard's Rho algorithm.

mod rho;
pub use rho::discrete_log;

#[cfg(test)]
mod tests;
