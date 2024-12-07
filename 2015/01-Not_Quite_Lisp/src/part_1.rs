use std::fs;

pub fn run(file_name: &str) -> i64 {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let line = input.lines().nth(0).unwrap();

    line.chars().fold(0, |mut sum, x| {
        sum += match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        sum
    })
}
