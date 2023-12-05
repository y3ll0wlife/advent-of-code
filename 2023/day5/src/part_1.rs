use std::{collections::HashMap, fs};

fn get_location(main: &HashMap<String, HashMap<usize, usize>>, seed_number: usize) -> usize {
    let soil = main["soil"]
        .get(&seed_number)
        .unwrap_or_else(|| &seed_number);

    let fertilizer = main["fertilizer"].get(&soil).unwrap_or_else(|| &soil);

    let water = main["water"]
        .get(&fertilizer)
        .unwrap_or_else(|| &fertilizer);

    let light = main["light"].get(&water).unwrap_or_else(|| &water);

    let temperature = main["temperature"].get(&light).unwrap_or_else(|| &light);

    println!("temperature{}", temperature);
    let humidity = main["humidity"]
        .get(&temperature)
        .unwrap_or_else(|| &temperature);

    humidity.clone()
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let seeds: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut counter = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();

    let mut main: HashMap<String, HashMap<usize, usize>> = HashMap::new();

    for line in input.lines() {
        if counter < 2 {
            counter += 1;
            continue;
        }

        if line.split(" ").count() != 3 {
            if line.is_empty() {
                continue;
            }

            let name = line.to_string().split("-").nth(0).unwrap().to_string();
            main.insert(name, map);
            map = HashMap::new();
            continue;
        }

        let sp: Vec<usize> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let destination_range = sp[0];
        let source_range = sp[1];
        let range = sp[2];

        let dest_range: Vec<usize> = (destination_range..(destination_range + range)).collect();
        let src_range: Vec<usize> = (source_range..(source_range + range)).collect();

        for i in 0..dest_range.clone().len() {
            map.insert(src_range[i], dest_range[i]);
        }
    }

    let mut lowest = 1000000000;
    for seed in seeds {
        let a = get_location(&main, seed);
        if a < lowest {
            lowest = a;
        }
    }

    lowest
}

/*
50 98 2
destination range start of 50    starts at 50 and includes [50, 51]
source range start of 98         starts at 98 and includes [98, 99]
range length of 2


seed number 98 -> soil number 50
seed number 99 -> soil number 51

52 50 48
source range  50 -> [50, 51, 52, 53...96,97]
destion range 52 -> [52, 53, 54, 55...98,99]

seed number 53 -> soil unmber 55

seed number = source range
soil number = destionion range
*/
