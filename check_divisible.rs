use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();

    let num: i32 = num.trim().parse().unwrap();

    if num % 5 == 0 {
        println!("{} is divisible by 5.", num);
    } else {
        println!("{} is not divisible by 5.", num);
    }
}