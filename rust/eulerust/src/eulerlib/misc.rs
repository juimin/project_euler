pub fn is_palindrome(u: u64) -> bool {
    let pal_str: Vec<char> = u.to_string().chars().collect();

    for idx in 0..pal_str.len() {
        if pal_str[idx] != pal_str[pal_str.len() - (idx + 1)] {
            return false
        }
    }

    return true
}

pub fn char_int_doubler(char_int: Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    let mut iterable_char_int = Vec::new();
    iterable_char_int.resize(char_int.len(), '0');
    iterable_char_int.copy_from_slice(&char_int);
    iterable_char_int.reverse();

    let mut carry: char = '0';

    for digit in iterable_char_int {
        let doubled: u32 = digit.to_digit(10).unwrap() * 2;
        result.push(char::from_digit((doubled % 10) + carry.to_digit(10).unwrap(), 10).unwrap());
        carry = char::from_digit(doubled / 10, 10).unwrap();
    }

    if carry != '0' {
        result.push(carry);
    }

    result.reverse();

    return result
}

pub fn char_int_adder(first: &Vec<char>, second: &Vec<char>) -> Vec<char> {
    let mut first_copy = Vec::new();
    first_copy.resize(first.len(), '0');
    let mut second_copy = Vec::new();
    second_copy.resize(second.len(), '0');

    first_copy.copy_from_slice(&first);
    first_copy.reverse();
    second_copy.copy_from_slice(&second);
    second_copy.reverse();

    // println!("Adding {:?} and {:?}", first, second);

    let mut iterations = first.len();
    if second.len() > first.len() {
        iterations = second.len();
    }

    // println!("Iterations: {}", iterations);

    let mut result = Vec::new();
    let mut carry = '0';
    for i in 0..iterations + 1 {
        let mut first_digit = '0';
        if i < first_copy.len() {
            first_digit = first_copy[i];
        }
        let mut second_digit = '0';
        if i < second_copy.len() {
            second_digit = second_copy[i];
        }

        let sum = first_digit.to_digit(10).unwrap() + second_digit.to_digit(10).unwrap() + carry.to_digit(10).unwrap();

        result.push(char::from_digit(sum % 10, 10).unwrap());
        carry = char::from_digit(sum / 10, 10).unwrap();

        // println!("chat_int_addr result: {:?}", result);
    }

    if *result.get(result.len() - 1).unwrap() == '0' {
        result.pop();
    }

    result.reverse();

    // println!("chat_int_addr final result: {:?}", result);

    return result
}

pub fn char_int_turtle_multiplier(num: &Vec<char>, multiples: i32) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    result.push('0');

    for _ in 0..multiples {
        result = char_int_adder(&result, num);
        // println!("char_int_turtle_multiplier: {:?}", result);
    }

    // println!("char_int_turtle_multiplier {:?} * {} = {:?}", num, multiples, result);

    return result
}