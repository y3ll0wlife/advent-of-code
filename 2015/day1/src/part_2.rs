use std::fs;

pub fn run(file_name: &str) -> i64 {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let line = input.lines().nth(0).unwrap();

    let mut char_num: i64 = 0;
    let mut sum = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '(' {
            sum += 1;
        } else {
            sum -= 1;
        }

        if sum == -1 {
            char_num = (i + 1) as i64;
            break;
        }
    }

    char_num
}
