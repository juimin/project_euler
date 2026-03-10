use std::collections::HashMap;

use eulerust::eulerlib::misc;

fn main() {
    let mut factorial_map: HashMap<u64,u64> = HashMap::new();
    for digit in 0..10 {
        factorial_map.insert(digit, misc::factorial(digit));
    }

    let mut final_sum = 0;

    let mut n = 3;
    while n < 10000000 {
        let digit_vec = misc::int_to_digit_vec(n);

        let mut sum_of_factorials = 0;
        for digit in digit_vec {
            sum_of_factorials += factorial_map[&(digit as u64)];
        }

        if n % 1 == 0 {
            println!("Debug: {} -> {}", n, sum_of_factorials);
        }

        if sum_of_factorials == n as u64 {
            println!("Found a curious number: {}", n);
            final_sum += sum_of_factorials;
        }

        n += 1;
    }

    println!("Final sum of numbers w/ curious property: {}", final_sum);
}