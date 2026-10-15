fn second_largest(nums: Vec<i32>) -> i32 {
    let mut largest = i32::MIN;
    let mut second = i32::MIN;
    let mut found = false;

    for num in nums {
        if num > largest {
            second = largest;
            largest = num;
            found = second != i32::MIN;
        } else if num < largest && num > second {
            second = num;
            found = true;
        }
    }

    if found {
        second
    } else {
        -1
    }
}

fn main() {
    let nums = vec![10, 5, 20, 8, 15];

    println!("{}", second_largest(nums));
}
