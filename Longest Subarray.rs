use std::collections::HashMap;

fn longest_distinct_subarray(nums: &[i32]) -> usize {
    let mut last_seen = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, &num) in nums.iter().enumerate() {
        if let Some(&prev_index) = last_seen.get(&num) {
            left = left.max(prev_index + 1);
        }

        last_seen.insert(num, right);

        max_len = max_len.max(right - left + 1);
    }

    max_len
}

fn main() {
    let nums = vec![1, 2, 3, 2, 4, 5];

    println!("{}", longest_distinct_subarray(&nums));
}
