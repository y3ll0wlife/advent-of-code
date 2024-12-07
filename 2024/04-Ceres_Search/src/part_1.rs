pub fn part1(input: Vec<&str>) -> usize {
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
            if grid[row][col] == 'X' {
                if row >= 3
                    && grid[row - 1][col] == 'M'
                    && grid[row - 2][col] == 'A'
                    && grid[row - 3][col] == 'S'
                {
                    total += 1
                }

                if row < (grid.len() - 3)
                    && grid[row + 1][col] == 'M'
                    && grid[row + 2][col] == 'A'
                    && grid[row + 3][col] == 'S'
                {
                    total += 1
                }

                if col >= 3
                    && grid[row][col - 1] == 'M'
                    && grid[row][col - 2] == 'A'
                    && grid[row][col - 3] == 'S'
                {
                    total += 1
                }

                if col < (grid[0].len() - 3)
                    && grid[row][col + 1] == 'M'
                    && grid[row][col + 2] == 'A'
                    && grid[row][col + 3] == 'S'
                {
                    total += 1
                }

                for i in [-1, 1].iter() {
                    for j in [-1, 1].iter() {
                        if row as i32 + i * 3 < grid.len() as i32
                            && col as i32 + j * 3 < grid[0].len() as i32
                            && row as i32 + i * 3 >= 0
                            && col as i32 + j * 3 >= 0
                            && grid[(row as i32 + i) as usize][(col as i32 + j) as usize] == 'M'
                            && grid[(row as i32 + i * 2) as usize][(col as i32 + j * 2) as usize]
                                == 'A'
                            && grid[(row as i32 + i * 3) as usize][(col as i32 + j * 3) as usize]
                                == 'S'
                        {
                            total += 1;
                        }
                    }
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
    fn part_1_test() {
        assert_eq!(
            part1(vec![
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
            18
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 2562);
    }
}
