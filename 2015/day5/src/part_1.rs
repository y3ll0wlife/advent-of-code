pub fn part1(input: &str) -> bool {
    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return false;
    }

    let vowels = input
        .chars()
        .filter(|x| x == &'a' || x == &'e' || x == &'i' || x == &'o' || x == &'u')
        .count();

    if vowels < 3 {
        return false;
    }

    let mut double_letter = false;
    for (i, c) in input.chars().enumerate() {
        let next = input.chars().nth(i + 1);
        if next.is_some() {
            if next.unwrap() == c {
                double_letter = true;
            }
        }
    }

    double_letter
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part1("ugknbfddgicrmopn"), true);
        assert_eq!(part1("aaa"), true);
        assert_eq!(part1("jchzalrnumimnmhp"), false);
        assert_eq!(part1("haegwjzuvuyypxyu"), false);
        assert_eq!(part1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn part_1() {
        let count = fs::read_to_string("./input.txt")
            .expect("Failed to read file")
            .lines()
            .filter(|x| part1(x))
            .count();

        assert_eq!(count, 236);
    }
}
