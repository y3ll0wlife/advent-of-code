use std::{collections::HashSet, fs};

fn main() {
    let p_2 = part_2();

    println!("{}", p_2);
    assert_eq!(p_2, 2497)
}

fn get_value(input: char) -> u32 {
    if input.is_uppercase() {
        return input as u32 - 65 + 27;
    }
    input as u32 - 97 + 1
}

fn get_shared(a: &str, b: &str, c: &str) -> char {
    let set: HashSet<char> = a.chars().collect();

    for b_char in b.chars() {
        for c_char in c.chars() {
            if set.contains(&b_char) && set.contains(&c_char) && b_char == c_char {
                return c_char;
            }
        }
    }

    'A'
}

fn part_2() -> u32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");

    let mut value = 0;

    let lines = input.lines().collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(3) {
        let first = lines.get(i).unwrap();
        let second = lines.get(i + 1).unwrap();
        let third = lines.get(i + 2).unwrap();

        value += get_value(get_shared(&first, &second, &third));
    }

    value
}
