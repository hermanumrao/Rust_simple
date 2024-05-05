use std::collections::HashMap;

// fn main() {
//     let mut state = HashMap::new(); // Corrected the call to new() method
//     state.insert("k", "Kerala");
//     state.insert("h", "Haryana");
//
//     println!("{:?}", state);
//     println!("size is {}", state.len());
// }


fn main() {
    let mut state=HashMap::new();
    state.insert("k","kerla");
    state.insert("h","herman");

    match state.get (&"kerla") {
        Some(expr) => {
            println!("value for key is {}",expr);
        } 
        None => {
            println!("nothing found");
        }
    }

    if state.contains_key("k"){
        println!("gotcha");
    }
    else {
        println!("not found");
    }
}
