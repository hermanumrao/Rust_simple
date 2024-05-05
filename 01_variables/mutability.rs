fn main() {
    let x=5;
    println!("x has value {x}");
//  Now if i write x=4 it will say err var x is not mutable
    let mut x=4;
    println!("x has value {x}");
    x=3; // now since x is mutable I can change values
    println!("x has value {x}");

}
