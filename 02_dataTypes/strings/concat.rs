fn main(){
   let mut first_name = "ber".to_string();
   first_name.push(' ');   
   let mut last_name = "ials".to_string(); 
   first_name.push_str(&last_name);
   println!("{}",first_name);
}

