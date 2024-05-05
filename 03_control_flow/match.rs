fn main() {
    let number = 4;
    match number {
        1 => println!("Baby"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("You are none of the above"),
    };
}

