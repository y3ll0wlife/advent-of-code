use std::collections::HashMap;

fn add_to_hash_map(num: usize, map: &mut HashMap<usize, usize>) {
    if let Some(count) = map.get_mut(&num) {
        *count += 1;
    } else {
        map.insert(num, 1);
    }
}

pub fn part2(input: Vec<&str>) -> usize {
    let mut left: HashMap<usize, usize> = HashMap::new();
    let mut right: Vec<usize> = vec![];

    for line in input {
        let (a, b) = line.split_once("   ").expect("failed to split once");
        right.push(a.parse().expect("failed to parse value"));
        add_to_hash_map(b.parse().expect("failed to parse value"), &mut left);
    }

    right
        .into_iter()
        .map(|number| {
            let left = left.get_key_value(&number).unwrap_or((&0, &0));
            number * left.1
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(
            part2(vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]),
            31
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 24643097);
    }
}
