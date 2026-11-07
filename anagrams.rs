use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

    for word in strs {
        let mut count = [0usize; 26];

        for ch in word.bytes() {
            count[(ch - b'a') as usize] += 1;
        }

        map.entry(count)
            .or_insert(Vec::new())
            .push(word);
    }

    map.into_values().collect()
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let result = group_anagrams(strs);

    for group in result {
        println!("{:?}", group);
    }
}
