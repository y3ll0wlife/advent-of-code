use std::collections::HashMap;

fn add_to_hash_map(key: i32, value: i32, map: &mut HashMap<i32, Vec<i32>>) {
    if let Some(count) = map.get_mut(&key) {
        count.push(value);
    } else if value == -1 {
        map.insert(key, vec![]);
    } else {
        map.insert(key, vec![value]);
    }
}

pub fn part1(input: Vec<&str>) -> i32 {
    let mut total = 0;

    let mut print_order: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut is_first_part = true;
    for line in input {
        if line.is_empty() {
            is_first_part = false;
            continue;
        }

        if is_first_part {
            let (n1, n2) = line.split_once("|").unwrap();
            let (num1, num2) = (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap());

            add_to_hash_map(num1, num2, &mut print_order);
        } else {
            let nums = line
                .split(",")
                .map(|f| f.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let mut valid_order = true;
            let mut printed_nums: Vec<i32> = vec![];
            for num in &nums {
                let (_, po) = match print_order.get_key_value(num) {
                    Some(value) => value,
                    None => {
                        add_to_hash_map(*num, -1, &mut print_order);
                        (&0, &vec![])
                    }
                };

                for b in po {
                    if printed_nums.contains(b) {
                        valid_order = false;
                        break;
                    }
                }

                if !valid_order {
                    break;
                } else {
                    printed_nums.push(*num);
                }
            }

            if valid_order {
                total += nums[nums.len() / 2];
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
    fn part_1_test() {
        assert_eq!(
            part1(vec![
                "47|53",
                "97|13",
                "97|61",
                "97|47",
                "75|29",
                "61|13",
                "75|53",
                "29|13",
                "97|29",
                "53|29",
                "61|53",
                "97|53",
                "61|29",
                "47|13",
                "75|47",
                "97|75",
                "47|61",
                "75|61",
                "47|29",
                "75|13",
                "53|13",
                "",
                "75,47,61,53,29",
                "97,61,53,29,13",
                "75,29,13",
                "75,97,47,61,53",
                "61,13,29",
                "97,13,75,29,47",
            ]),
            143
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 5955);
    }
}
