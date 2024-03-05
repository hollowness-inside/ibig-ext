use ibig::ops::RemEuclid;
use ibig::{ubig, UBig};
use jacobi::{Jacobi, JacobiValue};
use powmod::PowMod;

use crate::BinarySqrt;

/// Defines the `square_root_mod` function, which attempts to compute the square root of a number modulo another number.
pub trait SquareRootMod {
    /// Attempts to compute the square root of `self` modulo `modulo`.
    ///
    /// Returns `Some((root1, root2))` if there are two square roots, `None` otherwise.
    fn square_root_mod(self, modulo: &UBig) -> Option<(UBig, UBig)>;
}

impl SquareRootMod for UBig {
    fn square_root_mod(mut self, modulo: &UBig) -> Option<(UBig, UBig)> {
        self %= modulo;

        if self == ubig!(1) {
            return Some((ubig!(1), modulo - 1));
        }

        if self.jacobi(modulo.clone()) != Ok(JacobiValue::Residue) {
            return None;
        }

        if let Some(root) = self.sqrt() {
            return Some((root.clone(), modulo - root));
        }

        let (exp, odd) = get_k_s(modulo - 1);
        let exp = exp.max(2);

        let a1: UBig = self.powmod((&odd + 1) / 2, modulo);
        let a2: UBig = self.extended_gcd(modulo).1.try_into().unwrap();

        let mut n2 = ubig!(1);
        let n1 = {
            let mut i = ubig!(3);

            while i < (modulo - 2) {
                if i.jacobi(modulo.clone()) == Ok(JacobiValue::NonResidue) {
                    break;
                }

                i += 1;
            }

            i
        }
        .powmod(odd, modulo);

        for i in 0..=exp - 2 {
            let b: UBig = (&a1 * &n2) % modulo;
            let c: UBig = (&a2 * &b * b) % modulo;
            let d = c.powmod(ubig!(2).pow(exp - 2 - i), modulo);

            let ji = if d == ubig!(1) { 0 } else { 1 };
            n2 *= n1.pow(2usize.pow(i.try_into().unwrap()) * ji) % modulo;
        }

        let sol1: UBig = (a1 * n2) % modulo;
        if (&sol1 * &sol1).rem_euclid(modulo) != self {
            return None;
        }

        let sol2 = modulo - &sol1;

        Some((sol1, sol2))
    }
}

fn get_k_s(mut n: UBig) -> (usize, UBig) {
    let mut exponent = 1;

    while &n % ubig!(2) == ubig!(0) {
        n /= 2;
        exponent += 1;
    }
    (exponent, n)
}
