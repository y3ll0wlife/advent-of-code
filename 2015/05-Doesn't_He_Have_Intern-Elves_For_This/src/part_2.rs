use std::collections::HashSet;

pub fn part2(input: &str) -> bool {
    let mut pairs: HashSet<String> = HashSet::new();

    let mut letter_between = false;
    for (i, c) in input.chars().enumerate() {
        let double_space = input.chars().nth(i + 2);
        let next = input.chars().nth(i + 1);

        if next.is_some() {
            let pair = format!("{}{}", c, next.unwrap());
            if !pairs.contains(&pair) {
                pairs.insert(pair);
            }
        }

        if double_space.is_some() {
            if double_space.unwrap() == c {
                letter_between = true;
            }
        }
    }
    if !letter_between {
        return false;
    }

    let mut contains_pair = false;
    for pair in pairs {
        let (p1, p2) = input.split_once(&pair).unwrap();

        if p1.contains(&pair) || p2.contains(&pair) {
            contains_pair = true;
        }
    }
    contains_pair
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(part2("xxyxx"), true);
        assert_eq!(part2("uurcxstgmygtbstg"), false);
        assert_eq!(part2("ieodomkazucvgmuy "), false);
        assert_eq!(part2("aaa"), false);
        assert_eq!(part2("xxabcdefeghixx"), true);
        assert_eq!(part2("aaaa"), true);
        assert_eq!(part2("aaabcb"), false);
        assert_eq!(part2("aabacdefgaa"), true);
        assert_eq!(part2("zgsnvdmlfuplrubt"), false);
        assert_eq!(part2("xpbhhssgegmthwxb"), false);
        assert_eq!(part2("uqjzalppoorosxxo"), false);
        assert_eq!(part2("urrvucyrzzzooxhx"), false);
    }

    #[test]
    fn part_2() {
        let count = fs::read_to_string("./input.txt")
            .expect("Failed to read file")
            .lines()
            .filter(|x| part2(x))
            .count();

        assert_eq!(count, 51);
    }
}
