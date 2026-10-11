use std::io;

fn main() {
    let mut password = String::new();

    println!("Enter your password:");
    io::stdin().read_line(&mut password).unwrap();

    let password = password.trim();

    if password.len() < 8 {
        println!("Weak password: Use at least 8 characters.");
    } else if !password.chars().any(|c| c.is_ascii_digit()) {
        println!("Weak password: Add at least one number.");
    } else if !password.chars().any(|c| c.is_ascii_uppercase()) {
        println!("Weak password: Add an uppercase letter.");
    } else {
        println!("Your password meets the basic requirements!");
    }
}