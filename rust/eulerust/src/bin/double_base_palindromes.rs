use eulerust::eulerlib::misc;

fn main() {
    let mut sum = 0;

    for n in 0..1000000 {
        if misc::int_is_palindrome(n) {
            let binary_rep = misc::int_to_binary_vec(n);
            if misc::is_palindrome(&binary_rep) {
                println!("{} -> bin {}", n, String::from_iter(binary_rep));
                sum += n;
            }

        }
    }

    println!("Final sum: {}", sum);
}