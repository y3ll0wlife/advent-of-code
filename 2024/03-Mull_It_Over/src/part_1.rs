use regex::Regex;

pub fn part1(input: Vec<&str>) -> usize {
    let re = Regex::new(r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)").unwrap();

    let mut mults = 0;
    for line in input {
        let extract = re.captures_iter(line).map(|c| c.extract());
        for (_, [first, second]) in extract {
            mults += first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap();
        }
    }

    mults
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part1(vec![
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ]),
            161
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 170807108);
    }
}
