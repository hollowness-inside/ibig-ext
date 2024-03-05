#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::ubig;
use jacobi::Jacobi;

#[bench]
fn bench_jacobi(bench: &mut Bencher) {
    let x = ubig!(123456789);
    let y = ubig!(987654321);

    bench.iter(|| {
        for _ in 1..500 {
            black_box(x.jacobi(y.clone())).unwrap();
        }
    });
}
