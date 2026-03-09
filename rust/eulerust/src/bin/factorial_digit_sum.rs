use eulerust::eulerlib::misc;

fn main() {
    let target_factorial = 100;
    
    let mut result: Vec<char> = Vec::new();
    result.push('1');
    for multiplier in 1..target_factorial + 1 {
        result = misc::char_int_turtle_multiplier(&result, multiplier);
    }

    let mut final_sum = 0;
    for c in result {
        final_sum += c.to_digit(10).unwrap();
    }

    println!("{}! Digit sum: {:?}", target_factorial, final_sum);
}