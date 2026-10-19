use std::collections::HashMap;

fn most_frequent_word(text: &str) -> Option<(&str, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    counts.into_iter().max_by_key(|&(_, count)| count)
}

fn main() {
    let sentence = "apple orange banana apple orange apple";
    
    match most_frequent_word(sentence) {
        Some((word, count)) => println!("'{}' appeared {} times.", word, count),
        None => println!("No words found."),
    }

    assert_eq!(most_frequent_word(""), None);
}
