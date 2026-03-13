fn main() {
    let target_dimensions = 1001;

    let iterations =  4 * ((target_dimensions - 1) / 2);

    let mut rolling_sum = 1;
    let mut num = 1;
    let mut incremenet = 2;
    for i in 1..iterations + 1 {
        num += incremenet;
        println!("sum : {}", num);
        rolling_sum += num;
        if i % 4 == 0 {
            incremenet += 2;
        }
    }

    println!("Spiral sum: {}", rolling_sum);
}