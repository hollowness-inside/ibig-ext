use std::ops::Mul;

use ibig::ops::RemEuclid;
use ibig::UBig;

use num_traits::{One, Zero};

/// Defines the `powmod` method for modular exponentiation.
///
/// This trait provides a common interface for performing modular exponentiation
/// across `ibig` numeric data types that can be multiplied and have modulo operation defined for them.
pub trait PowMod {
    /// Calculates the modular exponentiation of `self` raised to the power of `exponent`
    /// modulo `divisor`.
    ///
    /// This method implements the efficient binary exponentiation algorithm to compute the
    /// result efficiently.
    ///
    /// # Arguments
    /// * `exponent`: The exponent to which `self` is raised. This should be an `UBig` type.
    /// * `divisor`: The modulus used for the modular exponentiation. This should be the same type as `self`.
    ///
    /// # Returns
    /// The result of the modular exponentiation, which is `self` raised to the power of `exponent` modulo `divisor`.
    fn powmod(&self, exponent: UBig, divisor: &Self) -> Self;
}

impl<T> PowMod for T
where
    T: One,
    for<'a> T: Mul<&'a T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T> + RemEuclid<&'a T, Output = T>,
{
    fn powmod(&self, mut exponent: UBig, divisor: &Self) -> Self {
        let mut base = self.rem_euclid(divisor);
        let mut result = Self::one();

        while exponent > UBig::zero() {
            if exponent.bit(0) {
                result = (result * &base).rem_euclid(divisor);
            }

            exponent >>= 1;
            base = (&base * &base).rem_euclid(divisor);
        }

        result
    }
}
