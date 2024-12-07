use std::fs;

#[derive(Debug)]
pub struct SetGame {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
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
        let sets = split.nth(1).unwrap();
        let cubes = convert_set_to_struct(sets);

        let mut minium_game = SetGame {
            red: 0,
            blue: 0,
            green: 0,
        };

        for cube in cubes {
            if cube.red > minium_game.red {
                minium_game.red = cube.red
            }
            if cube.blue > minium_game.blue {
                minium_game.blue = cube.blue
            }
            if cube.green > minium_game.green {
                minium_game.green = cube.green
            }
        }

        total += minium_game.red * minium_game.blue * minium_game.green
    }

    total
}
