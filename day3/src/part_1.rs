use std::fs;

fn main() {
    let p_1 = part_1();

    assert_eq!(p_1, 7872)
}

fn get_value(input: char) -> u32 {
    if input.is_uppercase() {
        return input as u32 - 65 + 27;
    }
    input as u32 - 97 + 1
}

fn part_1() -> u32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");

    let mut value = 0;

    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let mut already_check: Vec<char> = Vec::new();

        for first_char in first.chars().into_iter() {
            for second_char in second.chars().into_iter() {
                if first_char == second_char && !already_check.contains(&first_char) {
                    value += get_value(first_char);
                    already_check.push(first_char);
                }
            }
        }
    }
    value
}
