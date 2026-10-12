use std::io;

fn main() {
    let mut name = String::new();
    let mut marks1 = String::new();
    let mut marks2 = String::new();
    let mut marks3 = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter marks in Subject 1:");
    io::stdin().read_line(&mut marks1).unwrap();

    println!("Enter marks in Subject 2:");
    io::stdin().read_line(&mut marks2).unwrap();

    println!("Enter marks in Subject 3:");
    io::stdin().read_line(&mut marks3).unwrap();

    let m1: f64 = marks1.trim().parse().unwrap();
    let m2: f64 = marks2.trim().parse().unwrap();
    let m3: f64 = marks3.trim().parse().unwrap();

    let average = (m1 + m2 + m3) / 3.0;

    println!("\nStudent: {}", name.trim());
    println!("Average: {:.2}", average);

    if average >= 90.0 {
        println!("Grade: A");
    } else if average >= 75.0 {
        println!("Grade: B");
    } else if average >= 60.0 {
        println!("Grade: C");
    } else if average >= 40.0 {
        println!("Grade: D");
    } else {
        println!("Grade: F");
    }
}