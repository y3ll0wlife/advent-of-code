use std::collections::HashMap;

use regex::Regex;

pub fn part1(input: Vec<&str>) -> usize {
    let mut wiremap: HashMap<&str, u16> = HashMap::new();
    let operator_rg = Regex::new(r"(?<operator>[A-Z]+)").expect("invalid regex");

    for line in &input {
        let operator_cap = operator_rg.captures(line);
        if operator_cap.is_none() {
            let (value, wire) = line.split_once(" -> ").unwrap();

            let value = match value.parse::<u16>() {
                Ok(v) => v,
                Err(_) => {
                    if !wiremap.contains_key(value) {
                        wiremap.insert(value, 0);
                    }

                    wiremap.get_key_value(value).unwrap().1.clone()
                }
            };

            wiremap.insert(wire, value);
        } else {
            let operator = &operator_cap.unwrap()["operator"];
            match operator {
                "AND" => {
                    let mut split = line.split(" ");
                    let wire_a = split.nth(0).unwrap();
                    let wire_b = split.nth(1).unwrap();
                    let wire_location = split.nth(1).unwrap();

                    if !wiremap.contains_key(wire_a) {
                        wiremap.insert(wire_a, 0);
                    }
                    if !wiremap.contains_key(wire_b) {
                        wiremap.insert(wire_b, 0);
                    }

                    let wire_a_value = wiremap.get_key_value(wire_a).unwrap().1.clone();
                    let wire_b_value = wiremap.get_key_value(wire_b).unwrap().1.clone();
                    let wire_location_value = wire_a_value & wire_b_value;

                    if let Some(x) = wiremap.get_mut(wire_location) {
                        *x = wire_location_value;
                    } else {
                        wiremap.insert(wire_location, wire_location_value);
                    }

                    /*
                    println!(
                        "{} ({}) & {} ({}) -> {} ({})",
                        wire_a,
                        wire_a_value,
                        wire_b,
                        wire_b_value,
                        wire_location,
                        wire_location_value,
                    );
                     */
                }
                "OR" => {
                    let mut split = line.split(" ");
                    let wire_a = split.nth(0).unwrap();
                    let wire_b = split.nth(1).unwrap();
                    let wire_location = split.nth(1).unwrap();

                    if !wiremap.contains_key(wire_a) {
                        wiremap.insert(wire_a, 0);
                    }
                    if !wiremap.contains_key(wire_b) {
                        wiremap.insert(wire_b, 0);
                    }

                    let wire_a_value = wiremap.get_key_value(wire_a).unwrap().1.clone();
                    let wire_b_value = wiremap.get_key_value(wire_b).unwrap().1.clone();
                    let wire_location_value = wire_a_value | wire_b_value;

                    if let Some(x) = wiremap.get_mut(wire_location) {
                        *x = wire_location_value;
                    } else {
                        wiremap.insert(wire_location, wire_location_value);
                    }
                    /*
                    println!(
                        "{} ({}) | {} ({}) -> {} ({})",
                        wire_a,
                        wire_a_value,
                        wire_b,
                        wire_b_value,
                        wire_location,
                        wire_location_value,
                    );*/
                }
                "LSHIFT" => {
                    let mut split = line.split(" ");
                    let wire_a = split.nth(0).unwrap();
                    let value = split.nth(1).unwrap();
                    let wire_location = split.nth(1).unwrap();

                    if !wiremap.contains_key(wire_a) {
                        wiremap.insert(wire_a, 0);
                    }

                    let wire_a_value = wiremap.get_key_value(wire_a).unwrap().1.clone();
                    let wire_location_value = wire_a_value << value.parse::<i16>().unwrap();

                    if let Some(x) = wiremap.get_mut(wire_location) {
                        *x = wire_location_value;
                    } else {
                        wiremap.insert(wire_location, wire_location_value);
                    }

                    /*
                    println!(
                        "{} ({}) << {} -> {} ({})",
                        wire_a, wire_a_value, value, wire_location, wire_location_value,
                    );
                     */
                }
                "RSHIFT" => {
                    let mut split = line.split(" ");
                    let wire_a = split.nth(0).unwrap();
                    let value = split.nth(1).unwrap();
                    let wire_location = split.nth(1).unwrap();

                    if !wiremap.contains_key(wire_a) {
                        wiremap.insert(wire_a, 0);
                    }

                    let wire_a_value = wiremap.get_key_value(wire_a).unwrap().1.clone();
                    let wire_location_value = wire_a_value >> value.parse::<i16>().unwrap();

                    if let Some(x) = wiremap.get_mut(wire_location) {
                        *x = wire_location_value;
                    } else {
                        wiremap.insert(wire_location, wire_location_value);
                    }

                    /*
                    println!(
                        "{} ({}) >> {} -> {} ({})",
                        wire_a, wire_a_value, value, wire_location, wire_location_value,
                    );
                    */
                }
                "NOT" => {
                    let mut split = line.split(" ");
                    let wire_a = split.nth(1).unwrap();
                    let wire_location = split.nth(1).unwrap();

                    if !wiremap.contains_key(wire_a) {
                        wiremap.insert(wire_a, 0);
                    }

                    let wire_a_value = wiremap.get_key_value(wire_a).unwrap().1.clone();
                    let wire_location_value = !wire_a_value;

                    if let Some(x) = wiremap.get_mut(wire_location) {
                        *x = wire_location_value;
                    } else {
                        wiremap.insert(wire_location, wire_location_value);
                    }

                    /*
                    println!(
                        "!{} ({}) -> {} ({})",
                        wire_a, wire_a_value, wire_location, wire_location_value,
                    );
                     */
                }
                _ => panic!("invalid operator"),
            }
        }
    }

    0
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part1(vec![
                "123 -> x",
                "456 -> y",
                "x AND y -> d",
                "x OR y -> e",
                "x LSHIFT 2 -> f",
                "y RSHIFT 2 -> g",
                "NOT x -> h",
                "NOT y -> i"
            ]),
            0
        );
    }

    #[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("Failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 400410);
    }
}
