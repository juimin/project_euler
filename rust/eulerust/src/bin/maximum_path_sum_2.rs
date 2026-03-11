use eulerust::eulerlib::{files};

fn main() {
    let file_contents = files::read_file("67_triangle.txt");

    let rows = file_contents.split("\n");

    let mut parsed_rows: Vec<Vec<u64>> = Vec::new();
    for row in rows {
        let mut v = Vec::new();
        let nums = row.split(" ");
        for n in nums {
            v.push(u64::from_str_radix(n, 10).unwrap())
        }

        parsed_rows.push(v);
    }

    let mut aux: Vec<u64> = Vec::new();
    for row in parsed_rows.iter().rev() {
        let mut new_row: Vec<u64> = Vec::new();

        for (idx, value) in row.iter().enumerate() {
            let mut adder = 0;
            if idx < aux.len() && aux[idx] > adder {
                adder = aux[idx];
            }
            if idx + 1 < aux.len() && aux[idx + 1] > adder {
                adder = aux[idx + 1];
            }

            new_row.insert(idx, value + adder);
        }

        aux = new_row;
    }

    println!("{:?}", aux);
}