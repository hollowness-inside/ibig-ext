# ibig-ext
Compilation of mathematical libraries for the ibig crate

The library provides the following modules:
- Modular exponentiation `--features powmod`
- Jacobi symbol `--features jacobi`
- Tests for Primality `--features primality`
- Prime Generator `--features primegen`
- Pollard's $ρ$ and $ρ-1$ Factorization Algorithms `--features factorization`
- Pollard's Discrete Logarithm `--features logarithm`
- Modular Square Root `--features sqrt`


# Modular exponentiation `--features powmod`
Implementation of fast modular exponentiation for the `ibig` crate

## Example
```rust
use ibig::ubig;
use ibig_ext::powmod::PowMod;

fn main() {
    let base = ubig!(987654321);
    let exponent = ubig!(99999);
    let modulo = ubig!(123);

    let remainder = base.powmod(exponent, &modulo);

    println!("The remainder is {remainder}");
}

```

# Jacobi symbol `--features jacobi`
Module that allows to calculate the Jacobi symbol for `ibig` numeric types

The algorithm is described here,
https://www.mymathtables.com/numbers/legendre-symbol-generator.html

## Example
```rust
use ibig::ubig;
use ibig_ext::jacobi::Jacobi;

fn main() {
    let nominator = ubig!(100);
    let denominator = ubig!(9999999999);
    let j = nominator.jacobi(denominator.clone()).unwrap();

    println!("({nominator}|{denominator}) = {:?}", j as i8)
}
```

# Tests for Primality `--features primality`
Implementation of several algorithms for primality testing:
- [Fermat Primality Test](https://en.wikipedia.org/wiki/Fermat_primality_test)
- [Miller-Rabin Primality Test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
- [Solovay–Strassen Primality Test](https://en.wikipedia.org/wiki/Solovay%E2%80%93Strassen_primality_test)

## Examples
The implementation of all algorithms follows the same structure

```rust
use ibig::ubig;
use ibig_ext::primality::FermatTest;

fn main() {
    let to_check = ubig!(659555973512315526712786840633);
    let iterations: usize = 1000;

    let prob = to_check.fermat_test(iterations);
    println!("Probability of being prime is {prob:.2}");
}

```
> \>\>\> Probability of being prime is 1.00

There are also available `SolovayStrassenTest::solovay_strassen_test()` and `MillerRabinTest::miller_rabin_test()`

## Benchmarking
Benchmarked code 20 times, performing 100 primality checks per run.

### Fermat
```
     Running benches\fermat.rs (target\release\deps\fermat-bde67c468a64b7d5.exe)

running 1 test
test bench_fermat ... bench:  42,879,680 ns/iter (+/- 2,334,209)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 12.84s
```

### Miller-Rabin
```
     Running benches\mr.rs (target\release\deps\mr-f9d568e97228fa2b.exe)

running 1 test
test bench_mr ... bench:  44,263,650 ns/iter (+/- 2,668,341)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 13.32s
```

### Solovay-Strassen
```
     Running benches\ss.rs (target\release\deps\ss-412005c497779f6d.exe)

running 1 test
test bench_ss ... bench:  45,903,760 ns/iter (+/- 3,317,592)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 13.91s
```

# Prime Generator `--features primegen`
Generator of prime `ibig::UBig` numbers

## Example
```rust
use ibig_ext::prime_gen::gen_sized_prime;

fn main() {
    let bit_length = 1024;
    let prime = gen_sized_prime(bit_length, 10);

    println!("Random prime: {prime}");
    println!("Bit length: {}", prime.bit_len())
}
```

# Pollard's $ρ$ and $ρ-1$ Factorization Algorithms `--features factorization`
Implementation of [Pollard's Rho](https://en.wikipedia.org/w/index.php?title=Pollard%27s_rho_algorithm&oldid=1203127197) and [Rho Minus 1](https://en.wikipedia.org/w/index.php?title=Pollard%27s_p_%E2%88%92_1_algorithm&oldid=1191434534) algorithms for number factorization

## Examples
### $ρ$
```rust
use ibig::UBig;
use ibig_ext::factorization::pollards_rho;

fn main() {
    let n = UBig::from(2189524387u64) * UBig::from(7810462081u64) * UBig::from(6929615699u64);
    let c = UBig::from(152930000u64);

    match pollards_rho(&n, &c, 100000, |x| x * x + 1) {
        Some(factor) => println!("A non-trivial factor has been found: {}", factor),
        None => println!("No non-trivial divisor found."),
    }
}
```
***
### $ρ - 1$
```rust
use ibig::ubig;
use ibig_ext::factorization::pollards_rhom1;

fn main() {
    let n = ubig!(152936141) * 569225801 * 154819207;

    match pollards_rhom1(&n, 10000) {
        Some(factor) => println!("A non-trivial factor found: {}", factor),
        None => println!("No non-trivial divisor found."),
    }
}
```

# Pollard's Discrete Logarithm `--features logarithm`
Library that implements [Pollard's Rho algorithm for logarithms](https://en.wikipedia.org/w/index.php?title=Pollard%27s_rho_algorithm_for_logarithms&oldid=1188106187)

## Example
```rust
use ibig::ibig;

use ibig_ext::logarithm::discrete_log;
use ibig_ext::powmod::PowMod;

fn main() {
    let base = ibig!(4184873);
    let target = ibig!(1882449);
    let divisor = ibig!(4607807);

    let exponent = discrete_log(&base, &target, &divisor).unwrap();

    println!("{base} ^ {exponent} = {target} (mod {divisor})");
    println!("Check: {}", base.powmod(exponent, &divisor));
}
```

```
4184873 ^ 2817008 = 1882449 (mod 4607807)
Check: 1882449
```

# Modular Square Root `--features sqrt`
Library that implements an algorithm for calculating the modular square root

The algorithm solves the following equation for `x`, `x^2 = a (mod p)`, where `p` is prime

## Example
```rust
use ibig::ops::RemEuclid;
use ibig::ubig;

use ibig_ext::sqrt::SquareRootMod;

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
```
