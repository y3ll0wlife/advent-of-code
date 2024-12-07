use std::fs;

fn convert_name_to_digit(line: &str) -> char {
    match line {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;

    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut numbers: Vec<char> = vec![];

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                numbers.push(c);
                continue;
            };

            for (_, word) in nums.iter().enumerate().map(|(i, &word)| (i + 1, word)) {
                let substr = line.get(i..(i + word.len()));
                match substr {
                    Some(string) => {
                        if string == word {
                            numbers.push(convert_name_to_digit(string));
                        }
                    }
                    None => (),
                }
            }
        }

        let mut value = String::new();
        value.push(numbers.first().unwrap().clone());
        value.push(numbers.last().unwrap().clone());

        total += value.parse::<usize>().unwrap();
    }

    total
}
