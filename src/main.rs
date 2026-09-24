use std::io;

fn main() {
    let mut input = String::new();

    // Show all the operations available to the user and get the operation as an integer
    show_operations();
    println!("Enter the operation: ");
    io::stdin().read_line(&mut input).unwrap();
    let operation: i64 = input.trim().parse().unwrap();

    // Clear the input after every entry
    input.clear();

    // Get the first number from the user
    println!("Enter first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    input.clear();

    // Get the first number from the user
    println!("Enter second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    let result;

    if operation == 1 {
        result = add(num1, num2);
        println!("{:?} + {:?} = {:?}", num1, num2, result);
    } else if operation == 2 {
        result = subtract(num1, num2);
        println!("{:?} - {:?} = {:?}", num1, num2, result);
    } else {
        println!("Invalid operation!");
    }
}

fn show_operations() {
    println!(
        "Please 1 or 2 for following operations
    1. Addtion
    2. Subtraction"
    );
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}
