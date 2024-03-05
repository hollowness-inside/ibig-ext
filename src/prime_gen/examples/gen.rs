use prime_gen::gen_sized_prime;

fn main() {
    let bit_length = 1024;
    let prime = gen_sized_prime(bit_length, 10);

    println!("Random prime: {prime}");
    println!("Bit length: {}", prime.bit_len())
}
