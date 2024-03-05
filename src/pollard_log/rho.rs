use ibig::ops::RemEuclid;
use ibig::{ibig, IBig, UBig};
use powmod::PowMod;

/// Calculates the discrete logarithm of `target` with respect to `base` modulo `divisor` using the
/// Pollard's Rho algorithm.
///
/// Basically, it solves the equation, `base` ^ `x` = `target` (mod `divisor`) for `x`
///
/// # Arguments
///
/// * `base`: The base of the discrete logarithm.
/// * `target`: The value for which to find the discrete logarithm.
/// * `divisor`: The modulus for the discrete logarithm calculation.
///
/// # Returns
///
/// * `Some(x)`: The discrete logarithm `x`, where `base` raised to the power of `x` modulo `divisor` equals `target`.
/// * `None`: If no discrete logarithm exists for the given inputs.
pub fn discrete_log(base: &IBig, target: &IBig, divisor: &IBig) -> Option<UBig> {
    let q: IBig = (divisor - 1) / 2;

    let mut turtle = base * target;
    let mut ut = ibig!(1);
    let mut vt = ibig!(1);

    let mut rabbit = turtle.clone();
    let mut ur = ut.clone();
    let mut vr = vt.clone();

    let mut counter = ibig!(1);
    let limit = divisor - 1;

    while counter < limit {
        (turtle, ut, vt) = xab(&turtle, &ut, &vt, base, target, divisor, &q);
        (rabbit, ur, vr) = xab(&rabbit, &ur, &vr, base, target, divisor, &q);
        (rabbit, ur, vr) = xab(&rabbit, &ur, &vr, base, target, divisor, &q);

        if turtle == rabbit {
            break;
        }

        counter += 1;
    }

    let nom = ut - ur;
    let den = vr - vt;
    let res = (inverse(&den, &q) * nom).rem_euclid(&q);

    if let Ok(res) = TryInto::<UBig>::try_into(&res + &q) {
        if &base.powmod(res.clone(), divisor) == target {
            return Some(res);
        }
    }

    if let Ok(res) = TryInto::<UBig>::try_into(res) {
        if &base.powmod(res.clone(), divisor) == target {
            return Some(res);
        }
    }

    None
}

fn xab(x: &IBig, a: &IBig, b: &IBig, g: &IBig, h: &IBig, p: &IBig, q: &IBig) -> (IBig, IBig, IBig) {
    match x % 3 {
        0 => (x * g % p, (a + 1) % q, b.clone()),
        1 => (x * h % p, a.clone(), (b + 1) % q),
        2 => (x * x % p, a * 2 % q, b * 2 % q),
        _ => unreachable!(),
    }
}

fn inverse(a: &IBig, n: &IBig) -> IBig {
    a.extended_gcd(n).1
}
