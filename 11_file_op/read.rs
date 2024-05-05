use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("data.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    print!("{}", contents);
}

