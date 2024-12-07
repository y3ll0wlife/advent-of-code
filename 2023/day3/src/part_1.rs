use std::fs;

struct SchematicChar {
    y: usize,
    x: usize,
    character: char,
    is_digit: bool,
}
fn create_schematic_char(y: usize, x: usize, schematic: &Vec<Vec<char>>) -> SchematicChar {
    let character = schematic[y][x];

    SchematicChar {
        y,
        x,
        character,
        is_digit: character.is_digit(10),
    }
}

fn get_full_number(x: usize, schematic_row: &Vec<char>) -> usize {
    // check before
    let mut num = x;
    let mut before = String::new();
    while schematic_row.get(num).unwrap() != &'.' {
        if num == 0 {
            break;
        }
        num -= 1;
        if !schematic_row.get(num).unwrap().is_digit(10) {
            break;
        }

        before.push(schematic_row.get(num).unwrap().clone());
    }
    before = before.chars().rev().collect();

    // check after
    num = x;
    let mut after = String::new();
    while schematic_row.get(num).unwrap() != &'.' {
        num += 1;

        if schematic_row.get(num).is_none() {
            break;
        }

        if !schematic_row.get(num).unwrap().is_digit(10) {
            break;
        }

        after.push(schematic_row.get(num).unwrap().clone());
    }

    let value = format!("{}{}{}", before, schematic_row[x], after);
    String::from(value).parse().unwrap()
}

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total = 0;

    let symbols: Vec<char> = vec!['@', '&', '*', '$', '-', '#', '=', '%', '+', '/'];
    let mut schematic: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut schematic_row = vec![];
        for c in line.chars() {
            schematic_row.push(c);
        }
        schematic.push(schematic_row);
    }

    for (y, row) in schematic.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if !symbols.contains(c) {
                continue;
            }

            let top_left = create_schematic_char(y - 1, x - 1, &schematic);
            let top = create_schematic_char(y - 1, x, &schematic);
            let top_right = create_schematic_char(y - 1, x + 1, &schematic);

            let bottom_left = create_schematic_char(y + 1, x - 1, &schematic);
            let bottom = create_schematic_char(y + 1, x, &schematic);
            let bottom_right = create_schematic_char(y + 1, x + 1, &schematic);

            let left = create_schematic_char(y, x - 1, &schematic);
            let right = create_schematic_char(y, x + 1, &schematic);

            if top_left.is_digit && top.is_digit && top_right.is_digit {
                total += get_full_number(top_left.x, &schematic[top_left.y]);
            } else if top_left.is_digit && top.is_digit {
                total += get_full_number(top_left.x, &schematic[top_left.y]);
            } else if top_right.is_digit && top.is_digit {
                total += get_full_number(top_right.x, &schematic[top_right.y]);
            }

            if top_left.is_digit && !top.is_digit {
                total += get_full_number(top_left.x, &schematic[top_left.y]);
            }
            if top.is_digit && !top_left.is_digit && !top_right.is_digit {
                total += get_full_number(top.x, &schematic[top.y]);
            }
            if top_right.is_digit && !top.is_digit {
                total += get_full_number(top_right.x, &schematic[top_right.y]);
            }

            if bottom_left.is_digit && bottom.is_digit && bottom_right.is_digit {
                total += get_full_number(bottom_left.x, &schematic[bottom_left.y]);
            } else if bottom_left.is_digit && bottom.is_digit {
                total += get_full_number(bottom_left.x, &schematic[bottom_left.y]);
            } else if bottom_right.is_digit && bottom.is_digit {
                total += get_full_number(bottom_right.x, &schematic[bottom_right.y]);
            }

            if bottom_left.is_digit && !bottom.is_digit {
                total += get_full_number(bottom_left.x, &schematic[bottom_left.y]);
            }
            if bottom.is_digit && !bottom_left.is_digit && !bottom_right.is_digit {
                total += get_full_number(bottom.x, &schematic[bottom.y]);
            }
            if bottom_right.is_digit && !bottom.is_digit {
                total += get_full_number(bottom_right.x, &schematic[bottom_right.y]);
            }

            if left.is_digit {
                total += get_full_number(left.x, &schematic[left.y]);
            }

            if right.is_digit {
                total += get_full_number(right.x, &schematic[right.y]);
            }
        }
    }

    total
}
