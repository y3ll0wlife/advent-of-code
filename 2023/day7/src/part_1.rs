use std::{cmp::Ordering, collections::HashMap, fs, panic};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
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
    fn new(hand: HashMap<char, usize>) -> HandType {
        match hand.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                let mut h = HandType::FullHouse;
                for (_c, amount) in hand {
                    if amount == 4 {
                        h = HandType::FourOfAKind;
                        break;
                    }
                }

                h
            }
            3 => {
                let mut h = HandType::TwoPair;
                for (_c, amount) in hand {
                    if amount == 3 {
                        h = HandType::ThreeOfAKind;
                        break;
                    }
                }

                h
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
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

    let second_hand_chars: Vec<char> = b.chars().collect();
    let mut second_hand: HashMap<char, usize> = HashMap::new();
    for c in second_hand_chars.clone() {
        second_hand.entry(c).and_modify(|f| *f += 1).or_insert(1);
    }

    let first = HandType::new(first_hand);
    let second = HandType::new(second_hand);

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
