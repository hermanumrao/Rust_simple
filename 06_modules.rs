pub mod movies{
    pub fn printbs(){
        println!("hello");
    }
}

pub mod movie1{
    pub fn printbs1(){
        println!("hello");
    }
}

use movie1::printbs1;
fn main() {
    movies::printbs();
    printbs1();
}
