pub fn sum(values: Vec<u32>) -> u32 {
    values.iter().fold(0, |sum, value| { sum + value })
}
