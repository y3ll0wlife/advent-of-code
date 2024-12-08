use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

struct Grid {
    width: usize,
    height: usize,
}

fn add_to_hash_map(key: char, value: Pos, map: &mut HashMap<char, Vec<Pos>>) {
    if let Some(count) = map.get_mut(&key) {
        count.push(value);
    } else {
        map.insert(key, vec![value]);
    }
}

fn find_antinodes(pos1: Pos, pos2: Pos, grid: &Grid) -> Vec<Pos> {
    let mut res = vec![];
    let distance_x = pos1.x as i64 - pos2.x as i64;
    let distance_y = pos1.y as i64 - pos2.y as i64;

    let mut x = pos1.x as i64;
    let mut y = pos1.y as i64;

    while x >= 0 && x < grid.width as i64 && y >= 0 && y < grid.height as i64 {
        res.push(Pos {
            x: x as usize,
            y: y as usize,
        });

        x += distance_x;
        y += distance_y;
    }
    res
}

pub fn part2(input: Vec<&str>) -> usize {
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    let mut antinodes = HashSet::new();

    let grid = Grid {
        height: input.len(),
        width: input[0].len(),
    };

    for (x, row) in input.iter().enumerate() {
        for (y, col) in row.chars().enumerate() {
            if col != '.' {
                add_to_hash_map(col, Pos { x, y }, &mut antennas);
            }
        }
    }

    for group in antennas.values() {
        for (pos1, pos2) in group.iter().copied().tuple_combinations() {
            antinodes.extend(find_antinodes(pos1, pos2, &grid));
            antinodes.extend(find_antinodes(pos2, pos1, &grid));
        }
    }

    antinodes.len()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(
            part2(vec![
                "............",
                "........0...",
                ".....0......",
                ".......0....",
                "....0.......",
                "......A.....",
                "............",
                "............",
                "........A...",
                ".........A..",
                "............",
                "............"
            ]),
            34
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 1147);
    }
}
