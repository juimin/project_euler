use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut final_numerator = 1;
    let mut final_denominator = 1;

    for numerator in 10..100 {
        for denominator in 10..100 {
            if numerator == denominator {
                // Trivial == 1
                continue;
            }

            let n_tens = numerator / 10;
            let n_ones = numerator % 10;
            let d_tens = denominator / 10;
            let d_ones = denominator % 10;

            for (n_remove, d_remove, n_use, d_use) in vec!((n_tens,d_tens, n_ones, d_ones), (n_ones, d_ones, n_tens, d_tens), (n_tens,d_ones, n_ones, d_tens), (n_ones,d_tens, n_tens, d_ones)) {
                if n_remove + d_remove == 0 {
                    // Trivial zero ones place
                    continue;
                }
                if n_remove == d_remove {
                    if (n_use < d_use) && (n_use  as f64 / d_use  as f64) == (numerator as f64 / denominator as f64) {
                        // println!("Found curious fraction ({} / {}) == ({} / {}) ", numerator, denominator, n_use, d_use);
                        final_numerator *= n_use;
                        final_denominator *= d_use;
                    }
                }
            }
        }
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();


    println!("Final Fraction: {} / {} - took {:?}", final_numerator, final_denominator, (end - start).as_micros());
}