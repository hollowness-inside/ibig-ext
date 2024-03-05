#![feature(test)]

use std::str::FromStr;

use ibig::UBig;
use sqrt::BinarySqrt;

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_bin_sqrt(bench: &mut Bencher) {
    let number = UBig::from_str("103758195125389143237208135599207291373667583059449360785210149066856991891638975058427607259535179776").unwrap();

    bench.iter(|| black_box(number.sqrt()));
}
