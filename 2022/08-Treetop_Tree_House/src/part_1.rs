use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

fn visible_y(x: usize, y: usize, map: &Vec<Vec<usize>>) -> Vec<Direction> {
    let value = map[y][x];
    let height = map.len();
    let mut top_directions = vec![];
    let mut bottom_directions = vec![];

    for i in 0..y {
        if value <= map[i][x] {
            top_directions.clear();
            break;
        }

        if value > map[i][x] {
            top_directions.push(Direction::TOP);
        }
    }

    for i in (y + 1)..height {
        if value <= map[i][x] {
            bottom_directions.clear();
            break;
        }

        if value > map[i][x] {
            bottom_directions.push(Direction::BOTTOM);
        }
    }

    match top_directions.is_empty() && bottom_directions.is_empty() {
        true => vec![],
        false => [top_directions, bottom_directions].concat(),
    }
}

fn visible_x(x: usize, y: usize, map: &Vec<Vec<usize>>) -> Vec<Direction> {
    let value = map[y][x];
    let width = map[0].len();
    let mut left_directions = vec![];
    let mut right_directions = vec![];

    for i in 0..x {
        if value <= map[y][i] {
            left_directions.clear();
            break;
        }

        if value > map[y][i] {
            left_directions.push(Direction::LEFT);
        }
    }

    for i in (x + 1)..width {
        if value <= map[y][i] {
            right_directions.clear();
            break;
        }

        if value > map[y][i] {
            right_directions.push(Direction::RIGHT);
        }
    }

    match left_directions.is_empty() && right_directions.is_empty() {
        true => vec![],
        false => [left_directions, right_directions].concat(),
    }
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut map: Vec<Vec<usize>> = vec![];
    let mut visible_count = 0;

    for line in input.lines() {
        let mut map_line: Vec<usize> = vec![];

        for c in line.chars() {
            let value = c as usize - 0x30;
            map_line.push(value);
        }

        map.push(map_line);
    }

    let height = map.len();
    let width = map[0].len();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if !visible_x(x, y, &map).is_empty() || !visible_y(x, y, &map).is_empty() {
                visible_count += 1;
            }
        }
    }

    let border = map.len() * 4 - 4;
    border + visible_count
}
