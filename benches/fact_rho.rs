#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::UBig;
use ibig_ext::factorization::pollards_rho;

#[bench]
fn bench_rho(bench: &mut Bencher) {
    let n = UBig::from(2189524387u64) * UBig::from(7810462081u64) * UBig::from(6929615699u64);
    let c = UBig::from(152930000u64);

    bench.iter(|| {
        black_box(pollards_rho(&n, &c, 100000, |x| x * x + 1));
    });
}
