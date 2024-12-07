use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let mut blocks = input.split("\r\n\r\n");
    let seeds: Vec<usize> = blocks
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let maps: Vec<Vec<Vec<usize>>> = blocks
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|y| y.split(" ").map(|z| z.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    let value = seeds
        .into_iter()
        .map(|seed: usize| {
            maps.iter().fold(seed, |location, map| {
                for sub_map in map {
                    if location >= sub_map[1] && location < (sub_map[1] + sub_map[2]) {
                        return location - sub_map[1] + sub_map[0];
                    }
                }

                location
            })
        })
        .min()
        .unwrap();

    value
}
