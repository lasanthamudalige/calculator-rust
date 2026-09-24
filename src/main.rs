use std::io;

fn main() {
    let mut running: bool = true;

    while running {
        let mut input = String::new();

        // Show all the operations available to the user and get the operation as an integer
        show_operations();
        println!("Enter the operation: ");
        io::stdin().read_line(&mut input).unwrap();
        let operation: i64 = input.trim().parse().unwrap();

        // Clear the input after every entry
        input.clear();

        let result;

        if operation == 0 {
            running = false;
        } else if operation == 1 {
            let (num1, num2) = get_numbers();
            result = add(num1, num2);
            println!("\n{:?} + {:?} = {:?}\n", num1, num2, result);
        } else if operation == 2 {
            let (num1, num2) = get_numbers();
            result = subtract(num1, num2);
            println!("\n{:?} - {:?} = {:?}\n", num1, num2, result);
        } else {
            println!("\nInvalid operation!\n");
        }
    }
}

fn show_operations() {
    println!(
        "Please 0, 1 or 2 for following operations
    1. Addtion
    2. Subtraction
    0. To exit"
    );
}

fn get_numbers() -> (f64, f64) {
    let mut input = String::new();

    // Get the first number from the user
    println!("Enter first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    input.clear();

    // Get the first number from the user
    println!("Enter second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    // return 2 variables
    (num1, num2)
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2 //return the sum
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2 //return the difference
}
