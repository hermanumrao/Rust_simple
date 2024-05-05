
fn print(x: (i32, f64, u8)) {
    let (n1, n2, n3) = x; // Destructuring a tuple
    println!("{:?}", (n1, n2, n3));
}

fn main() {
    let tup = (10, 20, 30, 40, 50); // Define a tuple
    println!("Original tuple: {:?}", tup);
    
    let tuple: (i32, f64, u8) = (-325, 4.5, 22);
    print(tuple);
    println!("the 2nd element is: {}",tup.1);
}

