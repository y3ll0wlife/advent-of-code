use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let re = Regex::new(r"(?m)Card( )+(?P<card>[0-9]+): (?P<winning>[ 0-9]+)\| (?P<mine>[ 0-9]+)")
        .unwrap();

    let mut played = vec![0; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let caps = re.captures(line).unwrap();

        played[i] += 1;

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

        for w in 0..won.len() {
            played[i + w + 1] += played[i];
        }
    }

    played.iter().sum()
}
