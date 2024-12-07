pub fn part2(input: Vec<&str>) -> usize {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut total = 0;

    for index in input {
        let mut grid_x = vec![];
        for c in index.chars() {
            grid_x.push(c);
        }

        grid.push(grid_x);
    }

    for (row, row_char) in grid.iter().enumerate() {
        for (col, _) in row_char.iter().enumerate() {
            if grid[row][col] == 'A' {
                match row >= 1 && col >= 1 && col + 1 < grid[0].len() && row + 1 < grid.len() {
                    true => {
                        let top_left = grid[row - 1][col - 1];
                        let bottom_left = grid[row + 1][col - 1];

                        let top_right = grid[row - 1][col + 1];
                        let bottom_right = grid[row + 1][col + 1];

                        let valid_right = (top_left == 'M' && bottom_right == 'S')
                            || (top_left == 'S' && bottom_right == 'M');

                        let valid_left = (top_right == 'M' && bottom_left == 'S')
                            || (top_right == 'S' && bottom_left == 'M');

                        if valid_right && valid_left {
                            total += 1;
                        }
                    }
                    false => (),
                }
            }
        }
    }

    total
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(
            part2(vec![
                "MMMSXXMASM",
                "MSAMXMSMSA",
                "AMXSXMAAMM",
                "MSAMASMSMX",
                "XMASAMXAMM",
                "XXAMMXXAMA",
                "SMSMSASXSS",
                "SAXAMASAAA",
                "MAMMMXMMMM",
                "MXMXAXMASX",
            ]),
            9
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 1902);
    }
}
