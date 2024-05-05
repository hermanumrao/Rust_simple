#[derive(Debug)]
struct Emp {
    name: String,
    company:String,
    age:u32
}

fn main() {
    let emp1=Eemp{
        company:String::from("PES U"),
        name:String::from("bakara hun main"),
        age:50
    };
    println!("name {},{},{}",emp1.name,emp1.company,emp1.age);
}
