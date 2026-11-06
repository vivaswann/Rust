 
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();

    let mut answer = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&prev) = stack.last() {
            if temperatures[i] > temperatures[prev] {
                stack.pop();
                answer[prev] = (i - prev) as i32;
            } else {
                break;
            }
        }

        stack.push(i);
    }

    answer
}

fn main() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];

    let result = daily_temperatures(temperatures);

    println!("{:?}", result);
}
