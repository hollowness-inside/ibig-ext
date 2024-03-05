use ibig::{ubig, UBig};
use primality::MillerRabinTest;
use rand::Rng;

pub fn gen_sized_prime(bit_length: usize, persistence: usize) -> UBig {
    loop {
        let random = rand_sized_ubig(bit_length);
        if random.miller_rabin_test(persistence) >= 0.5 {
            break random;
        }
    }
}

fn rand_sized_ubig(bit_length: usize) -> UBig {
    let mut rng = rand::thread_rng();

    let start = ubig!(2).pow(bit_length - 1);
    let end = &start * 2;

    let mut result = rng.gen_range(start..end);
    result.set_bit(0);
    result
}

#[cfg(test)]
mod tests {
    use super::rand_sized_ubig;

    #[test]
    fn test_rand_sized_ubig() {
        for bit_length in 1..=1024 {
            let a = rand_sized_ubig(bit_length);
            assert_eq!(a.bit_len(), bit_length);
        }
    }
}
