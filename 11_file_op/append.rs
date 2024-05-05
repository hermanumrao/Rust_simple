use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("Failed to open file");
    file.write_all("rust class".as_bytes()).expect("Failed to write to file");
    println!("File appended");
}

