use ibig::ubig;

use ibig_ext::powmod::PowMod;

fn main() {
    let base = ubig!(987654321);
    let exponent = ubig!(99999);
    let modulo = ubig!(123);

    let remainder = base.powmod(exponent, &modulo);

    println!("The remainder is {remainder}");
}
