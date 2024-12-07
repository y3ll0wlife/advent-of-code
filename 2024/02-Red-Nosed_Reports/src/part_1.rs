pub fn part1(input: Vec<&str>) -> usize {
    let mut safe = 0;
    'main: for line in input {
        let mut split = line.split_whitespace();
        let first = split
            .next()
            .expect("failed to get next value")
            .parse::<i32>()
            .expect("failed to parse");
        let second = split
            .next()
            .expect("failed to get next value")
            .parse::<i32>()
            .expect("failed to parse");
        let is_increasing = first < second;
        let mut previous = second;

        if is_increasing {
            let diff = second - first;
            if !(1..=3).contains(&diff) {
                continue;
            }
        } else {
            let diff = first - second;
            if !(1..=3).contains(&diff) {
                continue;
            }
        }

        loop {
            let value = split.next();
            if value.is_none() {
                break;
            }
            let value = value.unwrap().parse::<i32>().expect("failed to parse");
            if is_increasing {
                let diff = value - previous;
                if !(1..=3).contains(&diff) {
                    continue 'main;
                }
            } else {
                let diff = previous - value;
                if !(1..=3).contains(&diff) {
                    continue 'main;
                }
            }

            previous = value;
        }
        safe += 1;
    }

    safe
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part1(vec![
                "7 6 4 2 1",
                "1 2 7 8 9",
                "9 7 6 2 1",
                "1 3 2 4 5",
                "8 6 4 4 1",
                "1 3 6 7 9"
            ]),
            2
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 202);
    }
}
