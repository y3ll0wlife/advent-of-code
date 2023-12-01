use core::panic;
use std::fs;

fn main() {
    let p_2 = part_2();

    println!("{}", p_2);

    assert_eq!(p_2, 13187);
}

fn convert_to_outcome(input: &str) -> &str {
    match input {
        "X" => "LOSE",
        "Y" => "DRAW",
        "Z" => "WIN",
        &_ => panic!("invalid input"),
    }
}

fn what_will_win_against(input: &str) -> &str {
    match input {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        &_ => panic!("invalid input"),
    }
}
fn what_will_lose_against(input: &str) -> &str {
    match input {
        "B" => "A",
        "C" => "B",
        "A" => "C",
        &_ => panic!("invalid input"),
    }
}

fn who_won(outcome_choicer: &str, opponent: &str) -> i32 {
    let mut points = 0;
    let outcome = convert_to_outcome(outcome_choicer);
    let mut you = "";

    if outcome == "DRAW" {
        you = opponent;
    } else if outcome == "LOSE" {
        you = what_will_lose_against(opponent);
    } else if outcome == "WIN" {
        you = what_will_win_against(opponent);
    }

    if you == opponent {
        points += 3;
    } else if you == "A" && opponent == "C" {
        points += 6;
    } else if you == "B" && opponent == "A" {
        points += 6;
    } else if you == "C" && opponent == "B" {
        points += 6;
    }

    if you == "A" {
        points += 1;
    } else if you == "B" {
        points += 2;
    } else if you == "C" {
        points += 3;
    }

    points
}

fn part_2() -> i32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");
    let mut total = 0;
    for line in input.lines() {
        let round = line.split(" ").collect::<Vec<&str>>();
        let opponent = round.get(0).unwrap();
        let outcome_choicer = round.get(1).unwrap();

        total += who_won(outcome_choicer, opponent);
    }

    total
}
