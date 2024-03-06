use ibig::ops::RemEuclid;
use ibig::{IBig, UBig};
use num_traits::{One, Zero};

use super::result::{Error, Result};

/// Possible values of the Jacobi symbol.
#[derive(Debug, PartialEq)]
pub enum JacobiValue {
    /// a is a non-residue modulo n.
    NonResidue = -1,

    /// a and n are not coprime
    NonCoprime = 0,

    /// a is a residue modulo n.
    Residue = 1,
}

/// Trait for calculating the Jacobi symbol.
pub trait Jacobi<Mod> {
    /// Calculates the Jacobi symbol (self|modulo).
    ///
    /// # Errors
    ///
    /// Returns `Err`:
    /// - If `modulo` is even.
    /// - If conversion errors occur during calculation.
    fn jacobi(&self, modulo: Mod) -> Result<JacobiValue>;
}

impl Jacobi<UBig> for UBig {
    /// Calculates the Jacobi symbol (self|modulo).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ibig::ubig;
    /// use ibig_ext::jacobi::{Jacobi, JacobiValue};
    ///
    /// let a = ubig!(17);
    /// let n = ubig!(5);
    ///
    /// assert_eq!(a.jacobi(n), Ok(JacobiValue::NonResidue));
    /// ```
    fn jacobi(&self, mut modulo: UBig) -> Result<JacobiValue> {
        if !modulo.bit(0) {
            return Err(Error::EvenModulo);
        }

        let mut base = self % &modulo;
        let mut sign = false;

        while !base.is_one() {
            if base.is_zero() {
                return Ok(JacobiValue::NonCoprime);
            }

            if base.bit(0) {
                if &base % 4 == 3 && &modulo % 4 == 3 {
                    sign = !sign;
                }

                std::mem::swap(&mut base, &mut modulo);
                base %= &modulo;
            } else {
                let tmp = &modulo % 8;
                if !(tmp == 1 || tmp == 7) {
                    sign = !sign;
                }

                base >>= 1;
            }
        }

        Ok(match sign {
            true => JacobiValue::NonResidue,
            false => JacobiValue::Residue,
        })
    }
}

impl Jacobi<UBig> for IBig {
    /// Calculates the Jacobi symbol (self|modulo).
    ///
    fn jacobi(&self, modulo: UBig) -> Result<JacobiValue> {
        let imod: IBig = modulo.clone().into();
        let base: UBig = self
            .rem_euclid(imod)
            .try_into()
            .map_err(|_| Error::ConversionError)?;

        base.jacobi(modulo)
    }
}
