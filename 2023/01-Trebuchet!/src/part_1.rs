use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;

    for line in input.lines() {
        let numbers = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();
        let char_1 = numbers.first().unwrap().clone();
        let char_2 = numbers.last().unwrap().clone();

        let mut value = String::new();
        value.push(char_1);
        value.push(char_2);

        total += value.parse::<usize>().unwrap();
    }

    total
}
