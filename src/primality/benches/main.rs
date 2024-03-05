#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

use ibig::ubig;
use primality::{FermatTest, MillerRabinTest, SolovayStrassenTest};

#[bench]
fn bench_fermat(bench: &mut Bencher) {
    let base = ubig!(659555973512315526712786840633);

    bench.iter(|| {
        black_box(base.fermat_test(100));
    });
}

#[bench]
fn bench_mr(bench: &mut Bencher) {
    let base = ubig!(659555973512315526712786840633);

    bench.iter(|| {
        black_box(base.miller_rabin_test(100));
    });
}

#[bench]
fn bench_ss(bench: &mut Bencher) {
    let base = ubig!(659555973512315526712786840633);

    bench.iter(|| {
        black_box(base.solovay_strassen_test(100));
    });
}
