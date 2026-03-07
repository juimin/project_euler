fn is_palindrome(u: u64) -> bool {
    let pal_str: Vec<char> = u.to_string().chars().collect();

    for idx in 0..pal_str.len() {
        if pal_str[idx] != pal_str[pal_str.len() - (idx + 1)] {
            return false
        }
    }

    return true
}

fn main() {
    let mut max: u64 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            let is_palindrome = is_palindrome(product);
            // println!("Num {} is palindrome: {}", product, is_palindrome);
            if is_palindrome && product > max {
                max = product
            }
        }
    }

    println!("Max palindromic product: {}", max)
}
