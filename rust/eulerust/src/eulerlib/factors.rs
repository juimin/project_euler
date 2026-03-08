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
        if n % x == 0 && x != 1 {
            factors.push(x);
        }
    }

    factors.sort();

    return factors
}

pub fn get_minimum_prime_factors(n: u64) -> Vec<u64> {
    let prime_factors = get_prime_factors(n);

    let mut target: u64 = n;
    let mut results: Vec<u64> = Vec::new();
    let mut trial_idx = 0;
    while target != 1 {
        if trial_idx > prime_factors.len() {
            panic!("Ran out of prime factors to use")
        }

        if target % prime_factors[trial_idx] == 0 {
            results.push(prime_factors[trial_idx]);
            target /= prime_factors[trial_idx];
        } else {
            trial_idx += 1;
        }
    }

    return results
    

}