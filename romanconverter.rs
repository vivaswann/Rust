use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter an integer (1-3999):");
    io::stdin().read_line(&mut input).unwrap();

    let num: u32 = match input.trim().parse() {
        Ok(n) if (1..=3999).contains(&n) => n,
        _ => {
            println!("Invalid input!");
            return;
        }
    };

    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut number = num;
    let mut result = String::new();

    for (value, symbol) in values {
        while number >= value {
            result.push_str(symbol);
            number -= value;
        }
    }

    println!("Roman numeral: {}", result);
}
