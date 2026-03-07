use eulerust::eulerlib::factors;


fn main() {
    let target: u64 = 600851475143;

    println!("Largest prime factor of {}: {}", target, factors::get_prime_factors(target).iter().max().unwrap())
}
