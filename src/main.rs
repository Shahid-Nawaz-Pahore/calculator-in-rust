use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero is not allowed!");
                0.0
            }
        }
    }
}

fn main() {
    // Input for the first number
    println!("Enter the first number:");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();
    let first_number: f64 = first_input.trim().parse().expect("Invalid number!");

    // Input for the operation
    println!("Enter the operation (+, -, *, /):");
    let mut operation_input = String::new();
    io::stdin().read_line(&mut operation_input).unwrap();
    let operation = operation_input.trim();

    // Input for the second number
    println!("Enter the second number:");
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).unwrap();
    let second_number: f64 = second_input.trim().parse().expect("Invalid number!");

    // Create an Operation enum instance
    let op = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    // Calculate and print the result
    let result = calculate(op);
    println!("Result: {}", result);
}
