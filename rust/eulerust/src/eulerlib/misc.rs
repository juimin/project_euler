pub fn is_palindrome(u: u64) -> bool {
    let pal_str: Vec<char> = u.to_string().chars().collect();

    for idx in 0..pal_str.len() {
        if pal_str[idx] != pal_str[pal_str.len() - (idx + 1)] {
            return false
        }
    }

    return true
}