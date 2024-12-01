use regex::Regex;

pub fn part2(input: Vec<&str>) -> i64 {
    let re = Regex::new(
        r"(?P<command>turn off|toggle|turn on) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)",
    )
    .expect("invalid regex");

    0
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2() {}
}
