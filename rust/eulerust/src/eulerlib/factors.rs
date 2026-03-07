use super::primes;

pub fn get_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    for x in 1..n + 1 {
        if n % x == 0 {
            factors.push(x);
        }
    }

    return factors
}

pub fn get_prime_factors(n: u64) -> Vec<u64> {
    let primes = primes::find_primes_sieve(n);
    
    let mut factors = Vec::new();
    for x in primes {
        if n % x == 0 {
            factors.push(x);
        }
    }

    return factors
}

