pub fn parse_numbers(values: Option<impl Iterator<Item = String>>) -> (i32, i32) {
    let numbers: Vec<i32> = values
        .unwrap()
        .map(|n| n.parse().expect("Invalid number"))
        .collect();
    (numbers[0], numbers[1])
}

pub fn sum_numbers(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}
pub fn multiplication_numbers(a: i32, b: i32) -> i32 {
    a * b
}
