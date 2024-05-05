use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("data.txt").expect("Failed to create file");
    file.write_all("Hello World".as_bytes()).expect("Failed to write to file");
    println!("Data written");
}

