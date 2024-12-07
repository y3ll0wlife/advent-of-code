use regex::Regex;

pub fn part2(input: Vec<&str>) -> i64 {
    let re = Regex::new(
        r"(?P<command>turn off|toggle|turn on) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)",
    )
    .expect("invalid regex");

    let mut grid = [[0i64; 1000]; 1000];

    for line in input {
        let caps = re.captures(line).expect("failed to capture line");
        let x1 = caps["x1"].parse::<usize>().unwrap();
        let y1 = caps["y1"].parse::<usize>().unwrap();
        let x2 = caps["x2"].parse::<usize>().unwrap();
        let y2 = caps["y2"].parse::<usize>().unwrap();

        match &caps["command"] {
            "turn on" => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        grid[x][y] += 1;
                    }
                }
            }
            "turn off" => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                }
            }
            "toggle" => {
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        grid[x][y] += 2
                    }
                }
            }
            _ => panic!("invalid command"),
        }
    }

    grid.iter()
        .flat_map(|a| a.iter())
        .cloned()
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 15343601);
    }
}
