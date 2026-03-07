use eulerust::eulerlib::primes;


fn largest_prime_factor(n: u64) -> u64 {
    let mut max_prime_factor = 1;

    let primes = primes::find_primes_sieve(n);
    
    for x in primes {
        if n % x == 0 {
            max_prime_factor = x;
        }
    }

    return max_prime_factor
}


fn main() {
    let target: u64 = 600851475143;
    println!("Largest prime factor of {}: {}", target, largest_prime_factor(target))
}
