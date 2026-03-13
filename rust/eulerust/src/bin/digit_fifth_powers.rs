use eulerust::eulerlib::misc;

fn main() {
    let mut rolling_sum = 0;
    for n in 2..10000000 {
        let mut sum = 0;
        for i in misc::int_to_digit_vec(n) {
            sum += i.pow(5);
            // println!("{} ^ {} = {}", i, 5, i.pow(5));
        }

        if sum == n {
            rolling_sum += sum;
            println!("sum of digits for {} = {}", n, sum);
            println!("Found the sum: {}", rolling_sum);
        }
        
    }
}