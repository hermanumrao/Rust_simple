use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Failed to remove file");
    println!("File removed successfully");
}

