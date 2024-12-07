use regex::Regex;

pub fn part2(input: Vec<&str>) -> usize {
    let re = Regex::new(
        r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)|(?P<do>do\(\))|(?P<dont>don't\(\))",
    )
    .unwrap();

    let mut mults = 0;
    let mut activated = true;

    for line in input {
        for cap in re.captures_iter(line) {
            if cap.name("do").is_some() {
                activated = true;
                continue;
            }

            if cap.name("dont").is_some() {
                activated = false;
                continue;
            }

            if activated {
                let (num1, num2) = (&cap["first"], &cap["second"]);
                mults += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
            }
        }
    }

    mults
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(
            part2(vec![
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ]),
            48
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 170807108);
    }
}
