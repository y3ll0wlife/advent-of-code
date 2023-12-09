use std::fs;

pub fn run(file_name: &str) -> i64 {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let instability_sensor: Vec<Vec<Vec<i64>>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|hist| {
            let mut diffs = vec![];
            diffs.push(hist.clone());

            while diffs.last().unwrap().iter().any(|x| *x != 0) {
                let diff = diffs.last().unwrap();
                let i: Vec<i64> = diff
                    .windows(2)
                    .map(|pair| (pair[0], pair[1]))
                    .map(|(a, b)| b - a)
                    .collect();

                diffs.push(i)
            }

            diffs
        })
        .collect();

    instability_sensor
        .iter()
        .map(|diffs| diffs.iter().rev().fold(0, |a, b| a + b.last().unwrap()))
        .sum()
}
