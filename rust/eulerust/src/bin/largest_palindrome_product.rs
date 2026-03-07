use eulerust::eulerlib::misc;

fn main() {
    let mut max: u64 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            let is_palindrome = misc::is_palindrome(product);
            // println!("Num {} is palindrome: {}", product, is_palindrome);
            if is_palindrome && product > max {
                max = product
            }
        }
    }

    println!("Max palindromic product: {}", max)
}
