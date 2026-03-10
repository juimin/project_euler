use std::collections::HashSet;

use eulerust::eulerlib::{factors};

fn main() {
    let mut amicables: HashSet<u64> = HashSet::new();
    let target = 10000;

    for a in 0..target {
        println!("Finding divisors for {}", a);
        let b = factors::sum_divisors(a,false);
        println!("Finding divisors for {}", b);
        let maybe_a = factors::sum_divisors(b, false);

       

        if a == maybe_a && a != b {
            amicables.insert(a);
            amicables.insert(b);

            println!("Amicable numbers?: {} {} {}",a ,b, maybe_a);
        }
    }

    println!("Amicables: {:?}", amicables);

    let mut amicable_sum = 0;
    for x in amicables {
        amicable_sum += x;
    }

   

    println!("Amicable Sum: {} under {}", amicable_sum, target);
}