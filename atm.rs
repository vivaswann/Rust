use std::io;

fn main() {
    let mut balance: f64 = 5000.0;

    loop {
        println!("\n===== ATM MENU =====");
        println!("1. Check Balance");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Your balance is Rs. {:.2}", balance);
            }

            "2" => {
                println!("Enter amount to deposit:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let amount: f64 = input.trim().parse().unwrap();

                if amount > 0.0 {
                    balance += amount;
                    println!("Deposit successful!");
                    println!("New balance: Rs. {:.2}", balance);
                } else {
                    println!("Enter a valid amount.");
                }
            }

            "3" => {
                println!("Enter amount to withdraw:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let amount: f64 = input.trim().parse().unwrap();

                if amount <= 0.0 {
                    println!("Enter a valid amount.");
                } else if amount > balance {
                    println!("Insufficient balance!");
                } else {
                    balance -= amount;
                    println!("Withdrawal successful!");
                    println!("Remaining balance: Rs. {:.2}", balance);
                }
            }

            "4" => {
                println!("Thank you for using the ATM!");
                break;
            }

            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
