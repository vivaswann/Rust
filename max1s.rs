fn main() {
    let nums = vec![1, 1, 0, 1, 1, 1];

    let mut count = 0;
    let mut max_count = 0;

    for num in nums {
        if num == 1 {
            count += 1;
            max_count = max_count.max(count);
        } else {
            count = 0;
        }
    }

    println!("{}", max_count);
}
