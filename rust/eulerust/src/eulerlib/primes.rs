pub fn find_primes_sieve(n: u64) -> Vec<u64> {
    // find all primes using the unoptimized sieve method
    let vec_size = (n + 1) as usize;
    let mut potential_primes: Vec<bool> = vec![true; vec_size];

    for num in 2..vec_size {
        let is_potential_prime: bool = potential_primes[num];
        if is_potential_prime && is_prime_trial_div(num as u64) {
            let mut tmp = num * 2;
            while tmp < (vec_size as usize) {
                potential_primes[tmp] = false;
                tmp += num;
            }
        }
    }

    let mut primes: Vec<u64> = Vec::new();
    for i in 1..vec_size {
        if potential_primes[i] {
            primes.push(i as u64);
        }
    }

    return primes

}





pub fn is_prime_trial_div(n: u64) -> bool {
    let mut val = 2;
    while (val * val) <= n {
        if n % val == 0 {
            return false
        }
        val += 1
    }

    return true
}