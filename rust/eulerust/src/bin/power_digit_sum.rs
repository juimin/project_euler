use eulerust::eulerlib::misc;

fn main() {
    let target = 1000;

    let mut latest: Vec<char> = Vec::new();
    latest.push('1');
    for _ in 1..target + 1 {
        latest = misc::char_int_doubler(latest);

    }

    // Calc sum of digits
    let mut sum = 0;
    for c in latest {
        sum += c.to_digit(10).unwrap();
    }

    println!("Sum of digits in 2 to the {} power : {}", target, sum)
}