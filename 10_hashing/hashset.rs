use std::collections::HashSet;

fn main() {
    let mut name = HashSet::new();
    name.insert("pes");
    name.insert("univ");
    name.insert("benga");
    println!("size= {}",name.len());
    println!("{:?}", name);

    for name_i in name.iter(){
        println!("{}",name_i);
    }
    match name.get(&"pes") {
        Some(expr) => {println!("found {}",expr); } // using get 
        None => {println!("niot found");}
    }
    if name.contains("pes")  {println!("found "); }
    else {println!("not found");}
}


