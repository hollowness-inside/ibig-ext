use ibig::UBig;
use pollard::pollards_rho;

fn main() {
    let n = UBig::from(2189524387u64) * UBig::from(7810462081u64) * UBig::from(6929615699u64);
    let c = UBig::from(152930000u64);

    match pollards_rho(&n, &c, 100000, |x| x * x + 1) {
        Some(factor) => println!("A non-trivial factor has been found: {}", factor),
        None => println!("No non-trivial divisor found."),
    }
}
