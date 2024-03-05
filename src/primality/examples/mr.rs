use ibig::ubig;
use primality::MillerRabinTest;

fn main() {
    let to_check = ubig!(659555973512315526712786840633);
    let iterations: usize = 1000;

    let prob = to_check.miller_rabin_test(iterations);
    println!("Probability of being prime is {prob:.2}");
}
