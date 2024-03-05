#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use prime_gen::gen_sized_prime;

#[bench]
fn bench_gen_sized_prime(bench: &mut Bencher) {
    let bit_length = 512;

    bench.iter(|| {
        let prime = gen_sized_prime(bit_length, 50);
        black_box(prime);
    });
}
