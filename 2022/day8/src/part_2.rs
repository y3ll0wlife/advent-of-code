use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    BLOCKED,
}

fn visible_y(x: usize, y: usize, map: &Vec<Vec<usize>>) -> (Vec<Direction>, Vec<Direction>) {
    let value = map[y][x];
    let height = map.len();
    let mut top_directions = vec![];
    let mut bottom_directions = vec![];

    for i in 0..y {
        if value <= map[i][x] {
            top_directions.push(Direction::BLOCKED);
            break;
        }

        if value > map[i][x] {
            top_directions.push(Direction::TOP);
        }
    }

    for i in (y + 1)..height {
        if value <= map[i][x] {
            bottom_directions.push(Direction::BLOCKED);
            break;
        }

        if value > map[i][x] {
            bottom_directions.push(Direction::BOTTOM);
        }
    }

    //bottom_directions.reverse();
    top_directions.reverse();

    (top_directions, bottom_directions)
}

fn visible_x(x: usize, y: usize, map: &Vec<Vec<usize>>) -> (Vec<Direction>, Vec<Direction>) {
    let value = map[y][x];
    let width = map[0].len();
    let mut left_directions = vec![];
    let mut right_directions = vec![];

    for i in 0..x {
        if value <= map[y][i] {
            left_directions.push(Direction::BLOCKED);
            break;
        }

        if value > map[y][i] {
            left_directions.push(Direction::LEFT);
        }
    }

    for i in (x + 1)..width {
        if value <= map[y][i] {
            right_directions.push(Direction::BLOCKED);
            break;
        }

        if value > map[y][i] {
            right_directions.push(Direction::RIGHT);
        }
    }

    //right_directions.reverse();
    left_directions.reverse();

    (left_directions, right_directions)
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut map: Vec<Vec<usize>> = vec![];

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
    let mut max = 0;

    for y in 0..height {
        for x in 0..width {
            let (left, right) = visible_x(x, y, &map);
            let (top, bottom) = visible_y(x, y, &map);

            let mut l = 0;
            let mut r = 0;
            let mut t = 0;
            let mut b = 0;

            for i in left {
                if i == Direction::BLOCKED {
                    l += 1;
                    break;
                }
                l += 1;
            }
            for i in right {
                if i == Direction::BLOCKED {
                    r += 1;
                    break;
                }
                r += 1;
            }
            for i in top {
                if i == Direction::BLOCKED {
                    t += 1;
                    break;
                }
                t += 1;
            }
            for i in bottom {
                if i == Direction::BLOCKED {
                    b += 1;
                    break;
                }
                b += 1;
            }
            let sum = l * r * t * b;

            if sum > max {
                println!("[{}] {} * {} * {} * {} = {}", y, t, l, b, r, sum);
                max = sum;
            }
        }
    }

    max
}
