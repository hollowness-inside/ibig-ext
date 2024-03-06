use ibig::{ubig, UBig};
use rand::Rng;

use crate::powmod::PowMod;

pub trait MillerRabinTest {
    fn miller_rabin_test(&self, iterations: usize) -> f64;
    fn miller_rabin_single_test(&self, base: &UBig) -> bool;
}

impl MillerRabinTest for UBig {
    fn miller_rabin_test(&self, iterations: usize) -> f64 {
        if self % 2 == 0 {
            return 0.0;
        }

        let mut rng = rand::thread_rng();
        let composites: f64 = (0..iterations).fold(0.0, |acc, _| {
            let a = rng.gen_range(ubig!(2)..self - 2);

            match self.miller_rabin_single_test(&a) {
                true => acc + 1.0,
                false => acc,
            }
        });

        composites / iterations as f64
    }

    fn miller_rabin_single_test(&self, base: &UBig) -> bool {
        let (s, r) = calculate_power_of_two(self - 1);
        let mut y = base.powmod(r, self);

        if y != ubig!(1) && y != self - 1 {
            let mut j = ubig!(1);

            while j <= (&s - 1) && y != self - 1 {
                y = y.powmod(ubig!(2), self);

                if y == ubig!(1) {
                    return false;
                }

                j += 1;
            }

            if y != self - 1 {
                return false;
            }
        }

        true
    }
}

fn calculate_power_of_two(mut n: UBig) -> (UBig, UBig) {
    let mut power = ubig!(0);

    while &n % 2 == 0 {
        power += 1;
        n /= 2;
    }

    (power, n)
}
