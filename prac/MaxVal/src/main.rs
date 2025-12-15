fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let int_slice = &nums[..];
    let result = max_value(int_slice);
    println!("Max value: {:?}", result);
}

fn max_value(slice: &[i32]) -> Option<i32> {
    let mut max = None;

    for &num in slice {
        max = Some(match max {
            None => num,
            Some(current_max) =>
                if num > current_max {
                    num
                }
                else {
                    current_max
                }
        });
    }
    max
}
