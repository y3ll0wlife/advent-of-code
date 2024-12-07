use std::fs;

use regex::Regex;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let re = Regex::new(r"(?m)Card( )+(?P<card>[0-9]+): (?P<winning>[ 0-9]+)\| (?P<mine>[ 0-9]+)")
        .unwrap();

    let mut copies_of_cards: Vec<usize> = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let card_id = &caps["card"].parse::<usize>().unwrap();
        copies_of_cards.push(card_id.clone());

        let copies: Vec<usize> = copies_of_cards.clone();
        let copies_card: Vec<&usize> = copies.iter().filter(|f| f.clone() == card_id).collect();

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

        for _i in copies_card {
            let mut wins = card_id.clone();
            for my_card in my_cards {
                if winning_cards.contains(my_card) {
                    wins += 1;
                    copies_of_cards.push(wins);
                }
            }
        }
    }

    copies_of_cards.len() as usize
}
