#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::ubig;
use ibig_ext::powmod::PowMod;

#[bench]
fn bench_pow(bench: &mut Bencher) {
    let base = ubig!(987654321);
    let exponent = ubig!(99999);
    let modulo = ubig!(123);

    bench.iter(|| {
        black_box(base.powmod(exponent.clone(), &modulo));
    });
}
