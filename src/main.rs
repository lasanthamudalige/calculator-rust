use std::io;

fn main() {
    let mut running: bool = true;

    while running {
        let mut input = String::new();

        // Show all the operations available to the user and get the operation as an integer
        show_operations();
        println!("Enter the operation: ");
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i64>() {
            // Show an error if the input is not an int64
            Ok(operation) => {
                // Clear the input after every entry
                input.clear();

                let result;

                if operation == 0 {
                    running = false; // stop the program by turning running varible to false
                } else if operation == 1 {
                    let (num1, num2) = get_numbers();
                    result = add(num1, num2);
                    println!("\n{:?} + {:?} = {:?}\n", num1, num2, result);
                } else if operation == 2 {
                    let (num1, num2) = get_numbers();
                    result = subtract(num1, num2);
                    println!("\n{:?} - {:?} = {:?}\n", num1, num2, result);
                } else if operation == 3 {
                    let (num1, num2) = get_numbers();
                    result = multiply(num1, num2);
                    println!("\n{:?} * {:?} = {:?}\n", num1, num2, result);
                } else if operation == 4 {
                    let (num1, num2) = get_numbers();
                    result = divide(num1, num2);
                    println!("\n{:?} / {:?} = {:?}\n", num1, num2, result);
                } else {
                    println!("\nPlease enter a valid operation.\n");
                }
            }
            Err(_) => {
                println!("\nPlease enter a valid operation.\n");
            }
        }
    }
}

fn show_operations() {
    println!(
        "Please 0, 1, 2, 3 or 4 for following operations
    1. Addtion
    2. Subtraction
    3. Multiplication
    4. Division
    0. To exit"
    );
}

fn get_numbers() -> (f64, f64) {
    // Get 2 numbers using get number function and return them
    let num1 = get_number("Enter first number: ");
    let num2 = get_number("Enter second number: ");

    (num1, num2)
}

fn get_number(message: &str) -> f64 {
    loop {
        let mut input = String::new();

        // Show the message to the number 1 or number 2
        println!("{}", message);
        io::stdin().read_line(&mut input).unwrap();

        // If the input is a valid number, the return the number or show the error message without
        // crashing
        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("\nInvalid number! Please try again.\n"),
        }
    }
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2 //return the sum
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2 //return the difference
}

fn divide(num1: f64, num2: f64) -> f64 {
    num1 / num2 //return the divided number
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2 //return the multiplied number
}
