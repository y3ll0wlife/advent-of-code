use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;
    let re = Regex::new(r"(?m)Card( )+(?P<card>[0-9]+): (?P<winning>[ 0-9]+)\| (?P<mine>[ 0-9]+)")
        .unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let winning_cards: &HashSet<usize> = &caps["winning"]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let my_cards: &HashSet<usize> = &caps["mine"]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let won: Vec<usize> = winning_cards.intersection(my_cards).copied().collect();
        if won.len() > 0 {
            total += u32::pow(2, (won.len() as u32) - 1);
        }
    }

    total as usize
}
