use eulerust::eulerlib::misc;

fn main() {
    // let v = vec!['1','0', '0'];
    // let res = misc::char_int_multiplier(&v, 10);
    // println!("Got Res: {:?}",res);
    // exit(0);

    let target = 1000;

    let mut result = Vec::new();
    result.push('0');

    for i in 1..target + 1 {
        let mut self_power: Vec<char> = i.to_string().chars().collect();
        for _ in 0..i - 1 {
            self_power = misc::char_int_multiplier(&self_power, i);
        }
        
        result = misc::char_int_adder(&result, &self_power);
        println!("Finished calculating self power of {}", i);
        // println!("Self power: {:?}", self_power);
    }

    let required_digits = 10;

    println!("Last {} digits of self power up to {} : {:?}", required_digits, target, &result[result.len()-required_digits..result.len()]);
}