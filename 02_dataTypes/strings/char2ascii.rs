fn main() {
    let secret_message = "xyz".to_string();
    let key =23;
    let mut mod_str = String::new();
    for c in secret_message.chars() {
        let mut asc = c as u8;
        asc+=key;
        mod_str.push_str(&asc.to_string());
    }
    println!("{}",mod_str);
}
