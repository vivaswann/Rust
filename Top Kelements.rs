
use std::collections::{HashMap, BinaryHeap};

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();

    for (num, count) in freq {
        heap.push((count, num));
    }

    let mut result = Vec::new();

    for _ in 0..k {
        if let Some((_, num)) = heap.pop() {
            result.push(num);
        }
    }

    result
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;

    let result = top_k_frequent(nums, k);

    println!("Top {} frequent elements: {:?}", k, result);
}
