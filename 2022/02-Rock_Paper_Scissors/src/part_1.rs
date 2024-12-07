use core::panic;
use std::fs;

fn main() {
    let p_1 = part_1();

    println!("{}", p_1);

    assert_eq!(p_1, 11449);
}

fn convert_to_same(input: &str) -> &str {
    match input {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        &_ => panic!("invalid input"),
    }
}

fn who_won(mut you: &str, opponent: &str) -> i32 {
    let mut points = 0;
    you = convert_to_same(you);

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

fn part_1() -> i32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");
    let mut total = 0;
    for line in input.lines() {
        let round = line.split(" ").collect::<Vec<&str>>();
        let opponent = round.get(0).unwrap();
        let you = round.get(1).unwrap();

        total += who_won(you, opponent);
    }

    total
}
