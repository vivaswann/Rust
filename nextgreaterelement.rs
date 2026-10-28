fn next_greater_element(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![-1; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&idx) = stack.last() {
            if nums[i] > nums[idx] {
                ans[idx] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }

        stack.push(i);
    }

    ans
}

fn main() {
    let nums = vec![2, 1, 5, 3, 4];

    let result = next_greater_element(nums);

    println!("{:?}", result);
}
