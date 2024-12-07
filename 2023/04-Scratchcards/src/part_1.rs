use std::fs;

use regex::Regex;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;
    let re = Regex::new(r"(?m)Card( )+(?P<card>[0-9]+): (?P<winning>[ 0-9]+)\| (?P<mine>[ 0-9]+)")
        .unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let winning_cards: &Vec<usize> = &caps["winning"]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let my_cards: &Vec<usize> = &caps["mine"]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut count: i32 = 0;
        for my_card in my_cards {
            if winning_cards.contains(my_card) {
                if count < 2 {
                    count += 1;
                } else {
                    count = count * 2;
                }
            }
        }
        total += count;
    }

    total as usize
}
