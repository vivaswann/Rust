fn is_palindrome(s: &str) -> bool {
    let cleaned_chars = s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase());

    cleaned_chars.clone().eq(cleaned_chars.rev())
}

fn main() {
    let test1 = "A man, a plan, a canal: Panama";
    let test2 = "hello world";
    let test3 = "No 'x' in Nixon";
    let test4 = "race a car";
    let test5 = " ";
    
    println!("'{}' is palindrome? {}", test1, is_palindrome(test1)); 
    println!("'{}' is palindrome? {}", test2, is_palindrome(test2)); 
    println!("'{}' is palindrome? {}", test3, is_palindrome(test3)); 
    println!("'{}' is palindrome? {}", test4, is_palindrome(test4)); 
    println!("'{}' is palindrome? {}", test5, is_palindrome(test5)); 
}
