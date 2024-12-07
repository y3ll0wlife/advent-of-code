use itertools::Itertools;

#[derive(Debug)]
enum Operators {
    Addition,
    Multiplication,
    Concatenator,
}

pub fn part2(input: Vec<&str>) -> u64 {
    let mut total = 0;
    for line in input {
        let (sum, numbers) = line.split_once(": ").unwrap();
        let sum = sum.parse::<u64>().unwrap();
        let numbers = numbers
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let operator_iter = (0..(numbers.len() - 1))
            .map(|_| {
                [
                    Operators::Addition,
                    Operators::Multiplication,
                    Operators::Concatenator,
                ]
                .iter()
            })
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        for opers in operator_iter {
            if numbers[1..]
                .iter()
                .zip(opers.into_iter())
                .fold(numbers[0], |total, (value, op)| match op {
                    Operators::Addition => total + value,
                    Operators::Multiplication => total * value,
                    Operators::Concatenator => total * 10u64.pow(value.ilog10() + 1) + value,
                })
                == sum
            {
                total += sum;
                break;
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
                "190: 10 19",
                "3267: 81 40 27",
                "83: 17 5",
                "156: 15 6",
                "7290: 6 8 6 15",
                "161011: 16 10 13",
                "192: 17 8 14",
                "21037: 9 7 18 13",
                "292: 11 6 16 20"
            ]),
            11387
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 61561126043536);
    }
}
