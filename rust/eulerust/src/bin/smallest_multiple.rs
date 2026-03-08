use eulerust::eulerlib::factors;
use std::collections::HashMap;

fn main() {
    let target = 20;

    let mut potential_factors: Vec<u64> = Vec::new();

    for n in 1..(target + 1) {
        potential_factors.push(n);
    }

    println!("Potein Factors: {:?}", potential_factors);

    let mut master_factor_map: HashMap<u64, u64> = HashMap::new();

    for factor in potential_factors {
        let minimum_prime_factors = factors::get_minimum_prime_factors(factor);
        
        let mut factor_map: HashMap<u64, u64> = HashMap::new();

        for pf in minimum_prime_factors{
            if factor_map.contains_key(&pf) {
                factor_map.insert(pf, factor_map[&pf] + 1); 
            } else {
                factor_map.insert(pf, 1); 
            }
        }

        for (key, value) in factor_map {
            if master_factor_map.contains_key(&key) {
                if value > master_factor_map[&key] {
                    master_factor_map.insert(key, value);
                }
            } else {
                master_factor_map.insert(key, value);
            }
        }
        
    }

    println!("Master Factor Map: {:?}", master_factor_map);

    let mut final_product = 1;
    for (key, value) in master_factor_map {
        final_product *= key.pow(value.try_into().unwrap());
    }

    println!("Smallest Multiple Found for {} : {}", target, final_product);

}
