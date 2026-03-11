use eulerust::eulerlib::files;

fn triangle(n: u64) -> u64 {
    return ((n + 1) * n) / 2;
}

fn is_triangle_number(triangle_number: u64) -> bool {
    // binary search it
    let mut low = 0;
    let mut high = triangle_number;
    
    while triangle_number > triangle(high) {
        high *= 2;
    }
    

    while high - low > 1 {

        let mid = (high + low) / 2;
        let mid_tn = triangle(mid);
        // println!("Mid {} (tn: {})", mid, mid_tn);
        if triangle_number == mid_tn {
            println!("Found triangle number for {} -> mid: {} mid_tn: {}", triangle_number, mid, mid_tn);
            return true;
        } else {
            if triangle_number > mid_tn {
                low = mid;
            } else {
                high = mid;
            }
        }
    }

    if triangle_number == triangle(high) {
        return true;
    }

    if triangle_number == triangle(low) {
        return true;
    }

    println!("Could not find for {} between {} (tn: {}) -> {} (tn: {})", triangle_number, low, triangle(low), high, triangle(high));
    return false;
}

fn word_value(s: &str) -> u64 {
    let mut value = 0;
    for char in s.chars() {
        println!("{} {}", char, char as u64 - 'A' as u64 + 1);
        value += char as u64 - 'A' as u64 + 1;
    }
    return value;
}

fn main() {
    let words = files::read_file("42_coded_triangle_numbers.txt");

    let split_words = words.split(",");

    let mut num_triangle_words = 0;
    for w in split_words {
        println!("{}", w);
        let value = word_value(&w[1..w.len()-1]);
        if is_triangle_number(value) {
            num_triangle_words += 1;
        }
    }

    println!("Num Triangle Words: {}", num_triangle_words);
}