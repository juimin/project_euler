use eulerust::eulerlib::files;

fn main() {
    let names = files::read_file("22_names_scores.txt");
    let names_split = names.split(",");

    let mut names_vec: Vec<&str> = Vec::new();
    for name in names_split {
        names_vec.push(&name[1..name.len()-1]);
    }

    names_vec.sort();

    // println!("{:?}", names_vec);

    let mut score_total: u64 = 0;

    //names_vec = Vec::new();
    //\names_vec.push("COLIN");

    for (i, name) in names_vec.iter().enumerate() {
        // println!("{} {}", i, name);


        let mut letter_sum = 0;
        for letter in name.chars() {
            letter_sum += letter as u32 - 'A' as u32 + 1;
        }
        
        let score: usize = letter_sum as usize * (i + 1);

        score_total += score as u64;

        println!("{}: {} * {} = {}", name, i + 1, letter_sum, score);
    }

    println!("Total Score: {}", score_total);


}