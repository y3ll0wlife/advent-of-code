use itertools::zip_eq;

pub fn part1(input: Vec<&str>) -> usize {
    let mut right: Vec<usize> = vec![];
    let mut left: Vec<usize> = vec![];

    for line in input {
        let mut split = line.split_whitespace();
        right.push(
            split
                .next()
                .expect("failed to find right")
                .parse()
                .expect("failed to convert to number"),
        );
        left.push(
            split
                .next()
                .expect("failed to find right")
                .parse()
                .expect("failed to convert to number"),
        );
    }

    right.sort_unstable();
    left.sort_unstable();

    zip_eq(left, right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part1(vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]),
            11
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 2769675);
    }
}
