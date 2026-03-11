use std::{fs::File, io::Read};

pub fn read_file(f: &str) -> String {
    let base_path: &str = "../../../resources";
    let mut file = File::open(format!("{}/{}", base_path, f)).unwrap();
    let mut contents = String::new();
    let res = file.read_to_string(&mut contents);

    if res.is_err() {
        panic!("something is bad");
    }

    return contents;
}