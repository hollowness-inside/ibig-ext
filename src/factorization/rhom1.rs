use ibig::{ubig, UBig};
use primes::{PrimeSet, Sieve};
use rand::Rng;

use crate::powmod::PowMod;

/// Attempts to find a non-trivial factor of a given number using Pollard's rho variant with the Montgomery function.
///
/// This implementation uses a smoothness bound to improve efficiency.
///
/// # Arguments
///
/// * `n`: The number to factorize.
/// * `smoothness`: The initial smoothness bound (a large number divisible only by small primes).
///
/// # Returns
///
/// * `Some(d)`: A non-trivial factor of `n` if found, where `d` is a `UBig`.
/// * `None`: If no factor is found within the smoothness bound.
///
pub fn pollards_rhom1(n: &UBig, mut smoothness: u64) -> Option<UBig> {
    while smoothness > 1 {
        let prod: UBig = Sieve::new()
            .iter()
            .take_while(|prime| prime <= &smoothness)
            .fold(ubig!(1), |acc, prime| {
                let div = (smoothness as f64).ln() / (prime as f64).ln();
                let exp = div.floor() as usize;
                let prime: UBig = prime.into();
                acc * prime.pow(exp)
            });

        let a = rand_coprime(n);
        let g = (a.powmod(prod, n) - ubig!(1)).gcd(n);

        if ubig!(1) < g && &g < n {
            return Some(g);
        }

        if g == ubig!(1) {
            smoothness -= 2;
        } else {
            smoothness += 2;
        }
    }

    None
}

fn rand_coprime(n: &UBig) -> UBig {
    let mut rng = rand::thread_rng();

    loop {
        let r = rng.gen_range(ubig!(2)..(n - 1));
        if r.gcd(n) == ubig!(1) {
            break r;
        }
    }
}
