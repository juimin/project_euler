use eulerust::eulerlib::misc;

fn main() {
    let target = 1000;

    let mut first: Vec<char> = Vec::new();
    first.push('0');
    let mut second: Vec<char> = Vec::new();
    second.push('1');


    // for _ in 0..target - 1 {
    //     let next = misc::char_int_adder(&first, &second);
    //     first = second;
    //     second = next;
    // }

    let mut fibonacci_number = 1;
    while second.len() < target {
        let next = misc::char_int_adder(&first, &second);
        first = second;
        second = next;
        fibonacci_number += 1;
    }

    println!("The first fibonacci number with over {} digits is #{} : {:?}", target, fibonacci_number, second);

}