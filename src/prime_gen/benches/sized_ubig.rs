#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::ubig;
use rand::Rng;

#[bench]
fn bench_bits(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let bit_length = 4096;

    bench.iter(|| {
        let mut result = ubig!(0);
        result.set_bit(bit_length - 1);
        result.set_bit(0);

        for i in 1..bit_length - 1 {
            if rng.gen() {
                result.set_bit(i);
            }
        }

        black_box(result)
    });
}

#[bench]
fn bench_range(bench: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let bit_length = 4096;

    bench.iter(|| {
        let start = ubig!(2).pow(bit_length - 1);
        let end = &start * 2;

        let mut result = rng.gen_range(start..end);
        result.set_bit(0);
        black_box(result)
    });
}
