use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part2(input: Vec<&str>) -> usize {
    let mut current_direction = Direction::Up;
    let mut x = 0;
    let mut y = 0;

    let mut grid: Vec<Vec<char>> = vec![];
    for (loc_x, row) in input.iter().enumerate() {
        let mut grid_row = vec![];
        for (loc_y, c) in row.chars().enumerate() {
            grid_row.push(c);

            if c == '^' {
                y = loc_y as i32;
                x = loc_x as i32;
            }
        }

        grid.push(grid_row);
    }

    let mut clean_grid = grid.clone();
    let start_x = x;
    let start_y = y;

    loop {
        match current_direction {
            Direction::Up => {
                if ((x + 1) as usize >= grid.len() || (x - 1) < 0)
                    || ((y + 1) as usize >= grid[0].len() || (y) < 0)
                {
                    break;
                }

                if grid[(x - 1) as usize][y as usize] == '#' {
                    current_direction = Direction::Right;
                    grid[x as usize][y as usize] = '>';
                    continue;
                }

                grid[x as usize][y as usize] = 'X';
                x -= 1;

                grid[x as usize][y as usize] = '^';
            }
            Direction::Down => {
                if ((x + 1) as usize >= grid.len() || (x - 1) < 0)
                    || ((y + 1) as usize >= grid[0].len() || (y) < 0)
                {
                    break;
                }

                if grid[(x + 1) as usize][y as usize] == '#' {
                    current_direction = Direction::Left;
                    grid[x as usize][y as usize] = '<';
                    continue;
                }

                grid[x as usize][y as usize] = 'X';
                x += 1;
                grid[x as usize][y as usize] = 'v';
            }
            Direction::Left => {
                if ((x + 1) as usize >= grid.len() || (x - 1) < 0)
                    || ((y + 1) as usize >= grid[0].len() || (y) < 0)
                {
                    break;
                }

                if grid[x as usize][(y - 1) as usize] == '#' {
                    current_direction = Direction::Up;
                    grid[x as usize][y as usize] = '^';
                    continue;
                }

                grid[x as usize][y as usize] = 'X';
                y -= 1;
                grid[x as usize][y as usize] = '<';
            }
            Direction::Right => {
                if ((x + 1) as usize >= grid.len() || (x - 1) < 0)
                    || ((y + 1) as usize >= grid[0].len() || (y) < 0)
                {
                    break;
                }

                if grid[x as usize][(y + 1) as usize] == '#' {
                    current_direction = Direction::Down;
                    grid[x as usize][y as usize] = 'v';
                    continue;
                }

                grid[x as usize][y as usize] = 'X';
                y += 1;
                grid[x as usize][y as usize] = '>';
            }
        }
    }

    let mut deadlocks = 0;

    for (row, i) in grid.iter().enumerate() {
        for (col, j) in i.iter().enumerate() {
            if j == &'X' {
                clean_grid[row][col] = '#';

                x = start_x;
                y = start_y;
                current_direction = Direction::Up;

                let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
                let mut was_deadlock = false;

                loop {
                    if visited.contains(&(x, y, current_direction.clone())) {
                        was_deadlock = true;
                        break;
                    }

                    visited.insert((x, y, current_direction.clone()));

                    match current_direction {
                        Direction::Up => {
                            if ((x + 1) as usize >= clean_grid.len() || (x - 1) < 0)
                                || ((y + 1) as usize >= clean_grid[0].len() || (y) < 0)
                            {
                                break;
                            }

                            if clean_grid[(x - 1) as usize][y as usize] == '#' {
                                current_direction = Direction::Right;
                                continue;
                            }

                            x -= 1;
                        }
                        Direction::Down => {
                            if ((x + 1) as usize >= clean_grid.len() || (x - 1) < 0)
                                || ((y + 1) as usize >= clean_grid[0].len() || (y) < 0)
                            {
                                break;
                            }

                            if clean_grid[(x + 1) as usize][y as usize] == '#' {
                                current_direction = Direction::Left;
                                continue;
                            }

                            x += 1;
                        }
                        Direction::Left => {
                            if ((x + 1) as usize >= clean_grid.len() || (x - 1) < 0)
                                || ((y + 1) as usize >= clean_grid[0].len() || (y - 1) < 0)
                            {
                                break;
                            }

                            if clean_grid[x as usize][(y - 1) as usize] == '#' {
                                current_direction = Direction::Up;
                                continue;
                            }

                            y -= 1;
                        }
                        Direction::Right => {
                            if ((x + 1) as usize >= clean_grid.len() || (x - 1) < 0)
                                || ((y + 1) as usize >= clean_grid[0].len() || (y) < 0)
                            {
                                break;
                            }

                            if clean_grid[x as usize][(y + 1) as usize] == '#' {
                                current_direction = Direction::Down;
                                continue;
                            }

                            y += 1;
                        }
                    }
                }

                clean_grid[row][col] = '.';
                if was_deadlock {
                    deadlocks += 1;
                }
            }
        }
    }

    deadlocks + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(
            part2(vec![
                "....#.....",
                ".........#",
                "..........",
                "..#.......",
                ".......#..",
                "..........",
                ".#..^.....",
                "........#.",
                "#.........",
                "......#...",
            ]),
            6
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 1836);
    }
}
