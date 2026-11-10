impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];

        dp[0] = 0;

        for i in 1..=n {
            for jump in 1..=3 {
                if i >= jump {
                    let cost = dp[i - jump]
                        + costs[i - 1]
                        + (jump * jump) as i32;

                    dp[i] = dp[i].min(cost);
                }
            }
        }

        dp[n]
    }
}
