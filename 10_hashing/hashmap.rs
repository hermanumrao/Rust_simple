fn main() {
    let number=4;
    let state = match number {
        1 => "baby",  // normal case
        2 |3|5|7|11 => println!("This is a prime"); // 2 or 3 or 5 or....
        13..=19 -> println!("a teen");  // any number between 13 to 19
        _ => println("your are none of the above")
    };
    println!("name={}",state);
}

