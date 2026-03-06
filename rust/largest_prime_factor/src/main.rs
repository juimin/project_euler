fn check_primaality_trial_division(n: u64) -> bool {
    let mut val = 2;
    while (val * val) <= n {
        if n % val == 0 {
            return false
        }
        val += 1
    }

    return true
}

fn largest_prime_factor(n: u64) -> u64 {
    let mut max_prime_factor = 1;

    let primes = find_primes_sieve(n);
    
    for x in primes {
        if n % x == 0 {
            max_prime_factor = x;
        }
    }

    return max_prime_factor
}

fn find_primes_sieve(n: u64) -> Vec<u64> {
    let vec_size = ((n + 1) as f64).sqrt().round() as usize;
    let mut potential_primes: Vec<bool> = vec![true; vec_size];

    println!("FInding potential primes in {}", n);

    for num in 2..vec_size {
        let is_potential_prime: bool = potential_primes[num];

        if is_potential_prime &&  check_primaality_trial_division(num as u64) {
            let mut tmp = num * 2;
            while tmp < (vec_size as usize) {
                potential_primes[tmp] = false;
                tmp += num;
            }
        }
    }

    println!("Finished finding potential primes");

    let mut primes: Vec<u64> = Vec::new();
    for i in 1..vec_size - 1 {
        if potential_primes[i] {
            primes.push(i as u64);
        }
    }

    println!("Found {} primes", primes.len());

    return primes

}


fn main() {
    let target: u64 = 600851475143;
    println!("Largest prime factor of {}: {}", target, largest_prime_factor(target))
}
