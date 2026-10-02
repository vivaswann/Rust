use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();

    let num: i32 = num.trim().parse().unwrap();

    if num % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}