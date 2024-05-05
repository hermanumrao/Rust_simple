#[derive(Debug)]
enum SexCategory {
    Male,
    Female,
}

#[derive(Debug)]
struct Person {
    sex: SexCategory, // change name and sex to _name and _sex to avoid warnings
    name: String,
}

fn main() {
    let p1 = Person {
        name: String::from("aka"),
        sex: SexCategory::Male,
    };
    println!("{:?}", p1);
}

