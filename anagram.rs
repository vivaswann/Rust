pub fn is_anagram(s: &str, t: &str) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    if s_bytes.len() != t_bytes.len() {
        return false;
    }

    let mut counts = [0i32; 26];

    for (&b_s, &b_t) in s_bytes.iter().zip(t_bytes.iter()) {
        counts[(b_s - b'a') as usize] += 1;
        counts[(b_t - b'a') as usize] -= 1;
    }

    counts.iter().all(|&c| c == 0)
}

fn main() {
    let test_cases = [
        ("anagram", "nagaram", true),
        ("rat", "car", false),
        ("listen", "silent", true),
        ("a", "ab", false),
        ("hello", "world", false),
    ];

    for (s, t, expected) in test_cases {
        let result = is_anagram(s, t);
        println!(
            "is_anagram(\"{}\", \"{}\") = {} (expected: {})",
            s, t, result, expected
        );
        assert_eq!(result, expected);
    }

    println!("\nAll test cases passed!");
}
