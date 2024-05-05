fn main() {
    let base_string = "hello ".to_string();
    let repetitions = 10;
    let mut i = 0;
    let mut repeated_string = "".to_string(); // Changed variable name to match usage in loop

    while i < repetitions {
        i += 1;
        repeated_string.push_str(&base_string);
    }
    println!("{}", repeated_string); // Print the repeated string instead of repetitions
}

