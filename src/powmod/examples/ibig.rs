use ibig::{ibig, ubig};
use powmod::PowMod;

fn main() {
    let base = ibig!(-6586423);
    let exponent = ubig!(99999);
    let modulo = ibig!(123);

    let remainder = base.powmod(exponent, &modulo);

    println!("The remainder is {remainder}");
}
