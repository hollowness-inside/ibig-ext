use ibig::{ubig, UBig};
use rand::Rng;

use powmod::PowMod;

pub trait FermatTest {
    fn fermat_test(&self, iterations: usize) -> f64;
    fn single_fermat_test(&self, base: &UBig) -> bool;
}

impl FermatTest for UBig {
    fn fermat_test(&self, iterations: usize) -> f64 {
        let mut rng = rand::thread_rng();

        let ones: f64 = (0..iterations).fold(0.0, |acc, _| {
            let a = rng.gen_range(ubig!(2)..self - 1);
            match self.single_fermat_test(&a) {
                true => acc + 1.0,
                false => acc,
            }
        });

        ones / iterations as f64
    }

    #[inline]
    fn single_fermat_test(&self, base: &UBig) -> bool {
        base.powmod(self - 1, self) == ubig!(1)
    }
}
