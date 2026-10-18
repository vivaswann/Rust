
use std::io;

fn encrypt(message: &str, shift: u8) -> String {
    let mut result = String::new();
    let shift = shift % 26;

    for ch in message.chars() {
        if ch.is_ascii_uppercase() {
            let new_char =
                (ch as u8 - b'A' + shift) % 26 + b'A';
            result.push(new_char as char);
        } else if ch.is_ascii_lowercase() {
            let new_char =
                (ch as u8 - b'a' + shift) % 26 + b'a';
            result.push(new_char as char);
        } else {
            result.push(ch);
        }
    }

    result
}

fn main() {
    let mut message = String::new();
    let mut input = String::new();

    println!("Enter your secret message:");
    io::stdin().read_line(&mut message).unwrap();

    println!("Enter shift value:");
    io::stdin().read_line(&mut input).unwrap();

    let shift: u8 = input.trim().parse().unwrap();

    let encrypted = encrypt(message.trim_end(), shift);

    println!("Encrypted Message: {}", encrypted);
}
