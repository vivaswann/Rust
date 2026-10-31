use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut balance = 0;
        let mut max_len = 0;

        map.insert(0, -1);

        for (i, num) in nums.iter().enumerate() {
            if num % 2 == 0 {
                balance += 1;
            } else {
                balance -= 1;
            }

            if let Some(&first_index) = map.get(&balance) {
                max_len = max_len.max(i as i32 - first_index);
            } else {
                map.insert(balance, i as i32);
            }
        }

        max_len
    }
}
