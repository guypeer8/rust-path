pub fn sum(values: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for value in values {
        sum += value;
    }
    sum
}