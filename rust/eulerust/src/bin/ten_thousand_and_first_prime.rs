use eulerust::eulerlib::primes;

fn main() {
    let target = 10001;

    // Prime number theorem says number of primes less than or equal to X is about X / ln(X).
    // So if we want to get the target within, we need target ~= X / ln(X)

    let mut highball: f64 = target as f64;

    while highball / highball.ln() < target as f64 {
        highball += target as f64;
    }

    println!("Estimating {} primes with X = {} to produce enough primes for {}", highball / highball.ln(), highball, target);

    let target_primes = primes::find_primes_sieve(highball.round() as u64);

    if target_primes.len() < target {
        panic!("Crap we don't have enough primes");
    }

    println!("The {} prime is : {}", target, target_primes[target]);

}