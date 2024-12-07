use std::{collections::HashMap, fs};

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut current_x = 0;
    let mut current_y = 0;
    let mut robo_current_x = 0;
    let mut robo_current_y = 0;
    let mut is_robo_turn = false;

    let mut houses = HashMap::new();
    houses.insert(format!("{}:{}", current_x, current_y), 2);

    let line = input.lines().nth(0).unwrap();
    for c in line.chars() {
        match is_robo_turn {
            true => match c {
                '>' => robo_current_x += 1,
                '<' => robo_current_x -= 1,
                '^' => robo_current_y += 1,
                'v' => robo_current_y -= 1,
                _ => panic!("invalid input"),
            },
            false => match c {
                '>' => current_x += 1,
                '<' => current_x -= 1,
                '^' => current_y += 1,
                'v' => current_y -= 1,
                _ => panic!("invalid input"),
            },
        }

        let key = match is_robo_turn {
            true => format!("{}:{}", robo_current_x, robo_current_y),
            false => format!("{}:{}", current_x, current_y),
        };

        if !houses.contains_key(&key) {
            houses.insert(key, 1);
        } else {
            *houses.get_mut(&key).unwrap() += 1;
        }

        is_robo_turn = !is_robo_turn;
    }

    houses.len()
}
