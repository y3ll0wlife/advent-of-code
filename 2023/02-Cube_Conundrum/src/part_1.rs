use std::fs;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Debug)]
pub struct SetGame {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl SetGame {
    pub fn too_many_cubes(&self) -> bool {
        if self.red > MAX_RED {
            return true;
        }
        if self.green > MAX_GREEN {
            return true;
        }
        if self.blue > MAX_BLUE {
            return true;
        }

        return false;
    }
}

fn convert_set_to_struct(set: &str) -> Vec<SetGame> {
    let values = set.split("; ").into_iter();
    let mut set_games = vec![];

    for game in values {
        let colors = game.split(", ").into_iter();
        let mut set_game = SetGame {
            red: 0,
            blue: 0,
            green: 0,
        };

        for color in colors {
            let value = color
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            if color.contains("red") {
                set_game.red = value;
            } else if color.contains("blue") {
                set_game.blue = value;
            } else if color.contains("green") {
                set_game.green = value;
            }
        }

        set_games.push(set_game);
    }

    set_games
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;

    for line in input.lines() {
        let mut split = line.split(": ");
        let game_id = split
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let sets = split.next().unwrap();
        let cubes = convert_set_to_struct(sets);

        let mut allowed_game: bool = true;
        for cube in cubes {
            if cube.too_many_cubes() {
                allowed_game = false;
            }
        }

        if allowed_game {
            total += game_id;
        }
    }

    total
}
