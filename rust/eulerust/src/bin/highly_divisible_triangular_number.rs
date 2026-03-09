use std::process::exit;

use eulerust::eulerlib::factors;

fn compute_triangluar_numbers(n :u64) -> Vec<u64> {
    let mut results: Vec<u64> = Vec::new();
    
    let mut last: u64 = 0;
    for x in 1..n + 1 {
        results.push(last + x);
        last = 0 + results.last().unwrap();
    }

    return results
}

fn main() {
    let threshold = 500;

    let triangular_numbers_limit = 1000000;
    let triangular_numbers = compute_triangluar_numbers(triangular_numbers_limit);
    // println!("Triangular Numbers: {:?}", triangular_numbers);

    for tn in triangular_numbers {
        let divisors = factors::count_divisors(tn);
        if divisors > 100 {
            println!("{} has {} factors", tn, divisors);
        }
        
        if divisors > threshold {
            println!("First triangular number with > {} factors is {} with {} factors", threshold, tn, divisors);
            exit(0)
        }
    }
}