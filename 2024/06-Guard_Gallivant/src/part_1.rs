#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: Vec<&str>) -> usize {
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

    grid.iter()
        .map(|f| f.iter().filter(|x| x == &&'X').count())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
        + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part1(vec![
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
            41
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 5461);
    }
}
