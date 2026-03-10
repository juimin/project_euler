pub fn is_palindrome(pal_str: &Vec<char>) -> bool {
    for idx in 0..pal_str.len() {
        if pal_str[idx] != pal_str[pal_str.len() - (idx + 1)] {
            return false
        }
    }

    return true
}

pub fn factorial(n: u64) -> u64 {
    let mut x = 1;

    for y in 1..n + 1 {
        x *= y;
    }

    return x;
}

pub fn int_to_binary_vec(n: u64) -> Vec<char> {
    let mut consumable = n;

    let mut result: Vec<char> = Vec::new();
    for _ in 0..64 {
        let bit = consumable & 1;
        if bit == 1 {
            result.push('1');
        } else {
            result.push('0');
        }
        consumable = consumable >> 1;
    }

    while result.len() > 0 && *result.last().unwrap() == '0' {
        result.pop();
    }

    result.reverse();

    return result;
}

pub fn int_is_palindrome(u: u64) -> bool {
    let pal_str: Vec<char> = u.to_string().chars().collect();
    return is_palindrome(&pal_str)
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

pub fn int_to_vec(n: u32) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    let mut m = n;
    while m != 0 {
        result.push(char::from_digit(m % 10, 19).unwrap());
        m = m / 10;
    }
    
    result.reverse();

    return result;
}

pub fn int_to_digit_vec(n: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let mut m = n;
    while m != 0 {
        result.push(m % 10);
        m = m / 10;
    }
    
    result.reverse();

    return result;
}

pub fn char_int_turtle_multiplier(num: &Vec<char>, multiples: u32) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    result.push('0');

    for _ in 0..multiples {
        result = char_int_adder(&result, num);
        // println!("char_int_turtle_multiplier: {:?}", result);
    }

    // println!("char_int_turtle_multiplier {:?} * {} = {:?}", num, multiples, result);

    return result
}

pub fn char_int_single_multiplier(first:  &Vec<char>, second_digit: char) -> Vec<char> {
    let mut first_copy = Vec::new();
    first_copy.resize(first.len(), '0');
    first_copy.copy_from_slice(&first);
    first_copy.reverse();

    if second_digit == '0' {
        return vec!['0'];
    }

    let iterations = first.len();

    let mut result = Vec::new();
    let mut carry = '0';
    for i in 0..iterations + 1 {
        let mut first_digit = '0';
        if i < first_copy.len() {
            first_digit = first_copy[i];
        }

        let sum = first_digit.to_digit(10).unwrap() * second_digit.to_digit(10).unwrap() + carry.to_digit(10).unwrap();

        result.push(char::from_digit(sum % 10, 10).unwrap());
        carry = char::from_digit(sum / 10, 10).unwrap();

        // println!("chat_int_addr result: {:?}", result);
    }

    while *result.last().unwrap() == '0' {
        result.pop();
    }

    result.reverse();
    
    // println!("char_int_single_multiplier final result: {:?} * {} = {:?}", first, second_digit, result);

    return result
}

pub fn char_int_multiplier(num: &Vec<char>, multiplier: u32) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let mut multi_vec = int_to_vec(multiplier);
    multi_vec.reverse();

    // println!("Multiplying {:?} by {:?}", num, multi_vec);

    for i in 0..multi_vec.len() {
        let mut sub_val: Vec<char> = Vec::new();

        let trial = char_int_single_multiplier(num, multi_vec[i]);
        sub_val.extend_from_slice(&trial);

        for _ in 0..i {
            sub_val.push('0');
        }

        // println!("sub_val final result {:?}", sub_val);

        result = char_int_adder(&result, &sub_val);
    }

    return result;
}