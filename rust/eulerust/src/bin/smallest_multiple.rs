use std::collections::HashSet;
use eulerust::eulerlib::factors;

fn main() {
    let target = 10;

    let mut potential_factors: HashSet<u64> = HashSet::new();

    for n in 1..(target + 1) {
        potential_factors.insert(n);
    }

    for n in 1..(target + 1) {
        if potential_factors.contains(&n) {
            let factors = factors::get_factors(n);
            for f in factors {
                if f != n && potential_factors.contains(&f) {
                    potential_factors.remove(&f);
                }
            }
        }
    }

    println!("Reduced Factors: {:?}", potential_factors);

    let mut prime_factor_set: HashSet<u64> = HashSet::new();

    for n in potential_factors {
        for pf in factors::get_prime_factors(n) {
            prime_factor_set.insert(pf);
        }

    }

    println!("Reduced Prime Factors: {:?}", prime_factor_set);


}
