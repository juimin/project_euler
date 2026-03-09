fn main() {
    let threshold: u64 = 1000000;

    let mut max_sequence_length_starting_number = 0;
    let mut max_sequence_length = 0;
    for x in 1..threshold + 1 {
        let mut moves = 0;
        let mut collatz = x;
        while collatz != 1 {
            if collatz % 2 == 0 {
                collatz = collatz / 2;
            } else {
                collatz = (collatz * 3) + 1;
            }
            moves += 1;
        }
        if moves > max_sequence_length {
            max_sequence_length = moves;
            max_sequence_length_starting_number = x;
        }
    }

    println!("Longest Collatz Sequence with starting values under {}: ({} -> {})", threshold, max_sequence_length_starting_number, max_sequence_length);
}