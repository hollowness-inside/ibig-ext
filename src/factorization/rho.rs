//! Provides an implementation of Pollard's rho algorithm for integer factorization.

use ibig::ops::RemEuclid;
use ibig::{ubig, UBig};

/// Attempts to find a non-trivial factor of a given number using Pollard's rho algorithm.
///
/// # Arguments
///
/// * `composite`: The number to factorize.
/// * `starting_value`: The starting value for the algorithm.
/// * `iterations`: The maximum number of iterations to perform.
/// * `f`: A function that generates a sequence of numbers for the algorithm.
///
/// # Returns
///
/// * `Some(d)`: A non-trivial factor of `n` if found, where `d` is a `UBig`.
/// * `None`: If no factor is found within the specified `iterations`.
///
pub fn pollards_rho<F>(
    composite: &UBig,
    starting_value: &UBig,
    iterations: usize,
    f: F,
) -> Option<UBig>
where
    F: Fn(&UBig) -> UBig,
{
    let f = |x: &UBig| f(x).rem_euclid(composite);

    let mut a = starting_value.clone();
    let mut b = starting_value.clone();
    let mut d = ubig!(1);

    let mut counter = 0;

    while d == ubig!(1) {
        a = f(&a);
        b = f(&f(&b));

        d = match a.ge(&b) {
            true => &a - &b,
            false => &b - &a,
        }
        .gcd(composite);

        counter += 1;
        if counter >= iterations {
            return None;
        }
    }

    (d > ubig!(1) && &d < composite).then_some(d)
}
