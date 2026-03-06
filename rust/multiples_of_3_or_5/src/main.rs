fn main() {
    let mut total = 0;
    let limit = 1000;
    for n in 0..limit {
        if n % 3 == 0 || n % 5 == 0  {
            total += n
        }
    }
    println!("Sum of values under limit {}: {}", limit, total)
}
