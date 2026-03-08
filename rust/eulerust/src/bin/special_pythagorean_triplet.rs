fn main() {
    for a in 1..1000 {
        let a_squared = a * a;

        for b in a..1000 {
            let b_squared = b * b;

            let c_squared = b_squared + a_squared;

            let maybe_c: f64 = (c_squared as f64).sqrt();
            
            if maybe_c % 1.0 == 0.0 {
                // println!("Found almost the special triplet: {} {} {}", a,b,maybe_c);
                if (a as f64 + b as f64 + maybe_c) == 1000.0 {
                    println!("Found the special triplet: {} {} {}", a,b,maybe_c);
                    println!("Product: {}", a * b * maybe_c as i32);
                }
            }

        }
    }
}