use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut beat_record: Vec<usize> = vec![];

    let times: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<usize> = input
        .lines()
        .nth(1)
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    for (time_index, t) in times.iter().enumerate() {
        let mut combos_won = 0;

        for i in 0..=t.clone() {
            let time_left = t - i;
            let distance = i * time_left;

            if distances[time_index] < distance {
                combos_won += 1;
            }
        }

        beat_record.push(combos_won);
    }

    beat_record.iter().fold(1, |acc, &x| acc * x)
}
