use std::collections::HashSet;

use eulerust::eulerlib::misc;

fn has_permuted_multiples(n: u32, multiples: u32) -> bool {
    let mut digit_set: HashSet<char> = HashSet::new();
    for digit in misc::int_to_vec(n) {
        digit_set.insert(digit);
    }

    for multiple in 2..multiples + 1 {
        let x = multiple * n;
        let mut temp_digit_set: HashSet<char> = HashSet::new();
        for digit in misc::int_to_vec(x) {
            temp_digit_set.insert(digit);
        }
        if !(digit_set == temp_digit_set) {
            return false;
        }
    }

    return true;

}

fn main() {
    let multiples = 6;
    let mut n: u32 = 1;

    while !has_permuted_multiples(n, multiples) {
        n += 1;
    }

    println!("Smallest integer with {} permuted multiples: {}", multiples, n);
}