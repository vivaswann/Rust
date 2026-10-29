fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),

            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }

            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }

            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }

            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    let s1 = "{[()]}";
    let s2 = "([)]";
    let s3 = "(((";

    println!("{}", is_balanced(s1)); // true
    println!("{}", is_balanced(s2)); // false
    println!("{}", is_balanced(s3)); // false
}
