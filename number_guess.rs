use std::io;

fn main() {
    let secret = 7;
    let mut guess = String::new();

    println!("Welcome to the Number Guessing Game!");
    println!("Guess a number between 1 and 10:");

    io::stdin().read_line(&mut guess).unwrap();

    let guess: i32 = guess.trim().parse().unwrap();

    if guess == secret {
        println!("Correct! You guessed the number.");
    } else if guess < secret {
        println!("Too low! The number was {}.", secret);
    } else {
        println!("Too high! The number was {}.", secret);
    }
}