fn main() {
    let mut total = 0;

    // Generate fibonacci in sequence
    let limit = 4000000;
    let mut n = 1;
    let mut m = 1;
    while m < limit {
        if m % 2 == 0 {
            total += m;
        }
        let sum = m + n;
        n = m;
        m = sum;
    }
    println!("Total of Even fibonacci numbers under {}: {}", limit, total)
}
