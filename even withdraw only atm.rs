/*An ATM has developed an attitude. It refuses to dispense money unless the withdrawal amount is even, divisible by 5, and not equal to 100.

Write a Rust program that takes a withdrawal amount and prints:
- "Cash dispensed!" if all conditions are satisfied.
- "Nice try, criminal." otherwise.
*/

fn withdraw(amount: i32) -> &'static str {
    if amount > 10000 {
        "Bro, this is an ATM, not a bank robbery."
    } else if amount > 0
        && amount % 2 == 0
        && amount % 5 == 0
        && amount != 100
    {
        "Cash dispensed!"
    } else {
        "Nice try, criminal."
    }
}

fn main() {
    println!("{}", withdraw(50));
    println!("{}", withdraw(100));
    println!("{}", withdraw(75));
    println!("{}", withdraw(120));
    println!("{}", withdraw(15000));
}
