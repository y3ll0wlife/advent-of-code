use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let times: Vec<&str> = input
        .lines()
        .nth(0)
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect();

    let time: usize = times.join("").parse().unwrap();

    let distances: Vec<&str> = input
        .lines()
        .nth(1)
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect();

    let distance: usize = distances.join("").parse().unwrap();
    let mut combos_won = 0;

    for i in 0..=time.clone() {
        let time_left = time - i;
        let distance_local = i * time_left;

        if distance < distance_local {
            combos_won += 1;
        }
    }

    combos_won
}
