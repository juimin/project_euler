fn main() {
    let limit:u32 = 100;

    let mut sum_of_squares: u32 = 0;
    for i in 1..limit + 1 {
        sum_of_squares += i.pow(2);
    }

    let mut sum: u32 = 0;
    for i in 1..limit + 1 {
        sum += i;
    }
    let square_of_sum = sum.pow(2);

    println!("Difference between sum of squares and square of sums = {}", square_of_sum - sum_of_squares);

}