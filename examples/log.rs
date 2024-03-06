use ibig::ibig;

use ibig_ext::pollard_log::discrete_log;
use ibig_ext::powmod::PowMod;

fn main() {
    let base = ibig!(4184873);
    let target = ibig!(1882449);
    let divisor = ibig!(4607807);

    let exponent = discrete_log(&base, &target, &divisor).unwrap();

    println!("{base} ^ {exponent} = {target} (mod {divisor})");
    println!("Check: {}", base.powmod(exponent, &divisor));
}
