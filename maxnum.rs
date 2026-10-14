impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if sum > num * 9 {
            return String::new();
        }

        let mut remaining = sum;
        let mut result = String::new();

        for _ in 0..num {
            let digit;

            if remaining >= 9 {
                digit = 9;
            } else {
                digit = remaining;
            }

            result.push_str(&digit.to_string());
            remaining -= digit;
        }

        result
    }
}
