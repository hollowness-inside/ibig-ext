//! This library provides implementations of Pollard's Rho algorithm for integer factorization.

mod rho;
pub use rho::pollards_rho;

mod rhom1;
pub use rhom1::pollards_rhom1;
