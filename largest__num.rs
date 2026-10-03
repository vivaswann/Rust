use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut a).unwrap();

    println!("Enter the second number:");
    io::stdin().read_line(&mut b).unwrap();

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();

    if a > b {
        println!("{} is larger.", a);
    } else if b > a {
        println!("{} is larger.", b);
    } else {
        println!("Both numbers are equal.");
    }
}