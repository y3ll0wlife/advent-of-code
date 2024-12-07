fn is_okey_list(list: Vec<i32>) -> bool {
    let is_increasing = list.first() < list.get(1);

    for (index, value) in list.iter().enumerate() {
        let next = match list.get(index + 1) {
            Some(t) => t,
            None => continue,
        };

        if is_increasing {
            let diff = next - value;
            if !(1..=3).contains(&diff) {
                return false;
            }
        } else {
            let diff = value - next;
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    }

    true
}

pub fn part2(input: Vec<&str>) -> usize {
    let mut safe = 0;
    for line in input {
        let split = line
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let status = is_okey_list(split.clone());

        if status {
            safe += 1;
            continue;
        }

        for index in 0..split.len() {
            let mut cloned_vec = split.clone();
            cloned_vec.remove(index);
            let status = is_okey_list(cloned_vec);

            if status {
                safe += 1;
                break;
            }
        }
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
            part2(vec![
                "7 6 4 2 1",
                "1 2 7 8 9",
                "9 7 6 2 1",
                "1 3 2 4 5",
                "8 6 4 4 1",
                "1 3 6 7 9"
            ]),
            4
        );
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 271);
    }
}
