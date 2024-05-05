use std::io;
fn main() {
    let mut line=String::new();
    println!("enter text :");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("hello, {}",line);
}
