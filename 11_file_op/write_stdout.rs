use std::io::Write;

fn main() {
    let b1 = std::io::stdout().write("PES ".as_bytes()).unwrap();
    let b2 = std::io::stdout().write("university".as_bytes()).unwrap();
    println!(); // Print a new line
}

