use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut num1).unwrap();

    println!("Enter second number:");
    io::stdin().read_line(&mut num2).unwrap();

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).unwrap();

    let num1: f64 = num1.trim().parse().unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    let result = match operator.trim() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    println!("Result: {}", result);
}