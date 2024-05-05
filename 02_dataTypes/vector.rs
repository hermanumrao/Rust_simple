fn slice_values(v: &[i32]) {
    let sliced_values = &v[1..4];
    println!("Sliced values: {:?}", sliced_values);
}

fn print_v(x: &[i32]) {
    println!("inside fn: {:?}", x);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    println!("inside: {:?}", v);
    v // returns valyse v
}

fn main() {
    let v = vec![10, 20, 30, 40, 50]; // Changed to a Vec<i32>
    print_v(&v);
    slice_values(&v);
    let v2 = v.clone(); 
    let r_v = display(v2);
    println!("Returned vector: {:?}", r_v);
}

