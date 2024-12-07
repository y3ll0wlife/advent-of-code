use std::{collections::HashMap, fs};

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
    let mut map: HashMap<&str, (String, String)> = HashMap::new();

    for line in input.lines().skip(2) {
        let (key, set) = line.split_once(" = ").unwrap();
        let cleaned_set = set.replace(&['(', ')'][..], "");
        let (a, b) = cleaned_set.split_once(", ").unwrap();

        map.insert(key, (a.to_string(), b.to_string()));
    }

    let mut current_location = String::from("AAA");
    let mut steps = 0;

    #[allow(while_true)]
    while true {
        for d in directions.clone() {
            match d {
                'R' => {
                    let tmp_map = &map.clone();
                    let val = &tmp_map.get(current_location.as_str()).unwrap().1;
                    current_location = val.clone();
                }
                'L' => {
                    let tmp_map = &map.clone();
                    let val = &tmp_map.get(current_location.as_str()).unwrap().0;
                    current_location = val.clone();
                }
                _ => panic!("Invalid input"),
            }

            steps += 1;
        }

        if current_location == "ZZZ" {
            break;
        }
    }

    steps
}
