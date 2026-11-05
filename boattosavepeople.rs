fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();

    let mut left = 0usize;
    let mut right = people.len() - 1;
    let mut boats = 0;

    while left <= right {
        if people[left] + people[right] <= limit {
            left += 1;
        }

        if left <= right {
            right -= 1;
        }

        boats += 1;
    }

    boats
}

fn main() {
    let people = vec![3, 2, 2, 1];
    let limit = 3;

    let result = num_rescue_boats(people, limit);

    println!("Minimum boats required: {}", result);
}
