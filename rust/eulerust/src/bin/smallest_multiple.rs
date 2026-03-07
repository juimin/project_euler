use std::collections::HashSet;
use eulerust::eulerlib::primes;

fn main() {
    let target = 10;

    let potential_factors: HashSet<u64> = HashSet::new();

    for n in (target + 1)..1 {
        potential_factors.insert(n);
    }

    for n in (target + 1)..1 {
        if potential_factors.contains(&n) {
            let prime_factors = primes::prime_factors(n);
            println!("Prime factors of {}, {:?}", n, prime_factors);
        }
    }

}
