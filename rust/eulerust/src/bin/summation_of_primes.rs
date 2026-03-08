use eulerust::eulerlib::primes;

fn main() {
    let target = 2000000;

    let mut sum: u64 = 0;

    for n in primes::find_primes_sieve(target) {
        sum += n;
    }

    // Don't count 1 as a prime
    println!("Sum of primes less than {} : {}", target, sum - 1);
}