use ibig::ubig;
use pollard::pollards_rhom1;

fn main() {
    let n = ubig!(152936141) * 569225801 * 154819207;

    match pollards_rhom1(&n, 10000) {
        Some(factor) => println!("A non-trivial factor found: {}", factor),
        None => println!("No non-trivial divisor found."),
    }
}
