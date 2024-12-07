use std::{cmp::Ordering, collections::HashMap, fs, panic};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(hand: HashMap<char, usize>, joker_count: usize) -> HandType {
        match hand.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let mut h = HandType::FourOfAKind;
                for (_c, amount) in hand {
                    if amount == 4 || amount == 1 {
                        h = match joker_count {
                            1 | 4 => HandType::FiveOfAKind,
                            _ => HandType::FourOfAKind,
                        };
                        break;
                    } else {
                        h = match joker_count {
                            2 | 3 => HandType::FiveOfAKind,
                            _ => HandType::FullHouse,
                        };
                        break;
                    }
                }

                h
            }
            3 => {
                let mut h = HandType::TwoPair;
                for (_c, amount) in hand {
                    if amount == 3 {
                        h = match joker_count {
                            1 | 3 => HandType::FourOfAKind,
                            _ => HandType::ThreeOfAKind,
                        };
                        break;
                    } else if amount == 2 {
                        h = match joker_count {
                            1 => HandType::FullHouse,
                            2 => HandType::FourOfAKind,
                            _ => HandType::TwoPair,
                        };
                        break;
                    }
                }

                h
            }
            4 => match joker_count {
                1 | 2 => HandType::ThreeOfAKind,
                _ => HandType::OnePair,
            },
            5 => match joker_count {
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            },
            _ => panic!("Invalid input"),
        }
    }
}

fn sort_hands(a: &str, b: &str) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }

    let first_hand_chars: Vec<char> = a.chars().collect();
    let mut first_hand: HashMap<char, usize> = HashMap::new();
    for c in first_hand_chars.clone() {
        first_hand.entry(c).and_modify(|f| *f += 1).or_insert(1);
    }
    let first_hand_jokers = first_hand.get(&'J').unwrap_or_else(|| &0).clone();

    let second_hand_chars: Vec<char> = b.chars().collect();
    let mut second_hand: HashMap<char, usize> = HashMap::new();
    for c in second_hand_chars.clone() {
        second_hand.entry(c).and_modify(|f| *f += 1).or_insert(1);
    }
    let second_hand_jokers = second_hand.get(&'J').unwrap_or_else(|| &0).clone();

    let first = HandType::new(first_hand, first_hand_jokers);
    let second = HandType::new(second_hand, second_hand_jokers);

    if first < second {
        return Ordering::Less;
    } else if first > second {
        return Ordering::Greater;
    }

    for i in 0..5 {
        if first_hand_chars[i] != second_hand_chars[i] {
            return Card::cmp(
                &Card::new(first_hand_chars[i]),
                &Card::new(second_hand_chars[i]),
            );
        }
    }

    Ordering::Equal
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let mut game: Vec<(&str, usize)> = vec![];
    input.lines().for_each(|x| {
        let (hand, bid) = x.split_once(" ").unwrap();

        game.push((hand, bid.parse().unwrap()))
    });
    game.sort_by(|a, b| sort_hands(&a.0, &b.0));

    let mut count = 0;
    let mut total = 0;
    for (_str, bid) in game {
        count += 1;

        total += bid * count;
    }

    total
}
