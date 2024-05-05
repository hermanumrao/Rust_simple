// fn process_array(arr: &[i32]) {
//     let mut sum_positive = 0;
//     let mut product_negative = 1;
//
//     for &num in arr.iter() {
//         if num > 0 {
//             sum_positive += num;
//         } else if num < 0 {
//             product_negative *= num;
//         }
//     }
//
//     println!("Sum of positive numbers: {}", sum_positive);
//     println!("Product of negative numbers: {}", product_negative);
// }
//
// fn main() {
//     let numbers = [3, -2, 7, -5, 1];
//     process_array(&numbers);
// }


fn slice_and_modify(arr: &mut [i32]) {
    let sliced_values = &arr[1..4]; 
    println!("Sliced values: {:?}", sliced_values);
    println!("Length after slicing: {}", sliced_values.len());

    if let Some(index) = sliced_values.iter().position(|&x| x == 30) {
        arr[index + 1] = 3000;
        println!("Modified{:?}", arr);
    } else {
        println!("30");
    }
}

fn main() {
    let mut arr = [10, 20, 30, 40, 50];
    slice_and_modify(&mut arr);
}
