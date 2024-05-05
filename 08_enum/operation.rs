enum Operation {
    Addition,
    Subtraction,
}

fn perform_operation(operation: Operation) -> i32 {
    match operation {
        Operation::Addition => 3 + 5,
        Operation::Subtraction => 3 - 5,
    }
}

fn main() {
    let result_addition = perform_operation(Operation::Addition);
    println!("Result of addition: {}", result_addition);

    let result_subtraction = perform_operation(Operation::Subtraction);
    println!("Result of subtraction: {}", result_subtraction);
}

