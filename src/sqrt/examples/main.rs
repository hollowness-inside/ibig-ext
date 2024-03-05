use ibig::ops::RemEuclid;
use ibig::ubig;
use sqrt::SquareRootMod;

fn main() {
    let div = ubig!(35239781);
    let a = ubig!(123);
    let (r1, r2) = a.clone().square_root_mod(&div).unwrap();

    println!("{r1}^2 = {r2}^2 = {a} (mod {div})");

    let check1 = (&r1 * &r1).rem_euclid(&div);
    let check2 = (&r2 * &r2).rem_euclid(&div);

    println!("Check 1: {r1}^2 = {check1} (mod {div})",);
    println!("Check 2: {r2}^2 = {check2} (mod {div})",);
}
