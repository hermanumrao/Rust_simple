fn main() {
    const MESSAGE1: &str = "Hello";
    const MESSAGE2: &str = "World";

    const SEPARATOR: usize = 5;

    let mixed_message = format!("{}{}{}", MESSAGE1, " ".repeat(SEPARATOR), MESSAGE2);

    println!("{}", mixed_message);
}

