use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Location {
    found: bool,
    location: String,
    steps: usize,
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm((&nums[1..]).to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

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

    let mut current_locations: Vec<Location> = map
        .keys()
        .filter(|x| x.chars().into_iter().last().unwrap() == 'A')
        .map(|f| Location {
            found: false,
            location: f.to_string(),
            steps: 0,
        })
        .collect();

    #[allow(while_true)]
    while true {
        for d in directions.clone() {
            for (i, location) in current_locations.clone().iter().enumerate() {
                if current_locations[i].found {
                    continue;
                }

                match d {
                    'R' => {
                        let tmp_map = &map.clone();
                        let val = &tmp_map.get(location.location.as_str()).unwrap().1;
                        current_locations[i].location = val.clone();
                    }
                    'L' => {
                        let tmp_map = &map.clone();
                        let val = &tmp_map.get(location.location.as_str()).unwrap().0;
                        current_locations[i].location = val.clone();
                    }
                    _ => panic!("Invalid input"),
                }

                current_locations[i].steps += 1;

                if current_locations[i].location.ends_with("Z") {
                    current_locations[i].found = true;
                }
            }
        }

        let non_found_count = current_locations.iter().filter(|x| !x.found).count();

        if non_found_count == 0 {
            break;
        }
    }

    let steps: Vec<usize> = current_locations.iter().map(|x| x.steps).collect();
    lcm(steps)
}
