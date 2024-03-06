use ibig::{ibig, ubig, IBig, UBig};
use rand::Rng;

use crate::jacobi::Jacobi;
use crate::powmod::PowMod;

pub trait SolovayStrassenTest {
    fn solovay_strassen_test(&self, iterations: usize) -> f64;
    fn solovay_strassen_single_test(&self, base: &UBig) -> bool;
}

impl SolovayStrassenTest for UBig {
    fn solovay_strassen_test(&self, iterations: usize) -> f64 {
        if self % 2 == 0 {
            return 0.0;
        }

        let mut rng = rand::thread_rng();
        let composites: f64 = (0..iterations).fold(0.0, |acc, _| {
            let a = rng.gen_range(ubig!(2)..self - 0);

            match self.solovay_strassen_single_test(&a) {
                true => acc + 1.0,
                false => acc,
            }
        });

        composites / iterations as f64
    }

    fn solovay_strassen_single_test(&self, base: &UBig) -> bool {
        let euler: IBig = base.powmod((self - 1) / 2, self).into();
        let jacobi: IBig = (base.jacobi(self.clone()).unwrap() as i8).into();

        let m1: IBig = IBig::from(self) - 1;
        (euler == m1 && jacobi == ibig!(-1)) || (euler == ibig!(1) && jacobi == ibig!(1))
    }
}
