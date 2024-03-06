use ibig::ubig;

use ibig_ext::primality::SolovayStrassenTest;

fn main() {
    let to_check = ubig!(659555973512315526712786840633);
    let iterations: usize = 1000;

    let prob = to_check.solovay_strassen_test(iterations);
    println!("Probability of being prime is {prob:.2}");
}
