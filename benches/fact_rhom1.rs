#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::ubig;
use ibig_ext::factorization::pollards_rhom1;

#[bench]
fn bench_rhom1(bench: &mut Bencher) {
    let n = ubig!(152936141) * 569225801 * 154819207;

    bench.iter(|| {
        black_box(pollards_rhom1(&n, 10000));
    });
}
