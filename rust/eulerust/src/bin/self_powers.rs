use eulerust::eulerlib::misc;

fn naive_main() {
    // let v = vec!['1','0', '0'];
    // let res = misc::char_int_multiplier(&v, 10);
    // println!("Got Res: {:?}",res);
    // exit(0);

    let target = 1000;
    let required_digits = 10;

    let mut result = Vec::new();
    result.push('0');

    for i in 1..target + 1 {
        let mut self_power: Vec<char> = i.to_string().chars().collect();
        for _ in 0..i - 1 {
            self_power = misc::char_int_multiplier(&self_power, i);
        }
        
        result = misc::char_int_adder(&result, &self_power);

        let mut truncated: Vec<char> = Vec::new();

        let mut stopping = required_digits;
        if result.len() < required_digits {
            stopping = result.len();
        }

        for _ in 0..stopping {
            truncated.push(result.pop().unwrap());
        }

        truncated.reverse();

        result = truncated;

        println!("Finished calculating self power of {}", i);
        // println!("Self power: {:?}", self_power);
    }



    println!("Last {} digits of self power up to {} : {:?}", required_digits, target, &result[result.len()-required_digits..result.len()]);
}


fn mod_pow(mut base: u128, mut exp: u32, modulus: u128) -> u128 {
    let mut result = 1;
    while exp > 0 {
        println!("Calculating e:{} b:{} r:{}", exp, base, result);
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}

fn main() {
    const MOD: u128 = 10_000_000_000;
    
    let mut res: u128 = 0;
    for i in 1..=1000 {
        res = (res + mod_pow(i, i as u32, MOD)) % MOD;
    }
    println!("{}", res);
}