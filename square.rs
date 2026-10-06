use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut num).unwrap();

    let num: i32 = num.trim().parse().unwrap();

    let square = num * num;

    println!("The square of {} is {}", num, square);
}