use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();

    let num: u32 = num.trim().parse().unwrap();
    let mut fact = 1;

    for i in 1..=num {
        fact *= i;
    }

    println!("Factorial of {} is {}", num, fact);
}