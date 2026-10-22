fn main() {
    let mut x = 5;
    let mut y = 10;

    println!("Before: x = {}, y = {}", x, y);

    x = x + y; 
    y = x - y; 
    x = x - y;

    println!("After: x = {}, y = {}", x, y);
}
