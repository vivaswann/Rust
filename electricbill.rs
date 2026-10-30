
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter electricity units consumed:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let units: u32 = input.trim().parse().expect("Invalid input");

    let bill: u32;

    if units <= 100 {
        bill = units * 2;
    } else if units <= 200 {
        bill = (100 * 2) + ((units - 100) * 3);
    } else {
        bill = (100 * 2) + (100 * 3) + ((units - 200) * 5);
    }

    println!("Total Electricity Bill: Rs. {}", bill);
}
