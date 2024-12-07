use std::fs;

fn main() {
    let p_1 = part_1();
    let p_2 = part_2();

    assert_eq!(p_1, 69310);
    assert_eq!(p_2, 206104);
}

fn part_1() -> i32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");

    let mut most_calories = 0;
    let mut curr: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line == "" {
            let total: i32 = curr.iter().sum();

            if most_calories < total {
                most_calories = total;
            }

            curr.clear();

            continue;
        }

        let calories = line.parse::<i32>().expect("failed to convert line to i32");
        curr.push(calories);
    }

    most_calories
}

fn part_2() -> i32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");

    let mut ret = 0;
    let mut total: Vec<i32> = Vec::new();
    let mut curr: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line == "" {
            let curr_total: i32 = curr.iter().sum();
            total.push(curr_total);

            curr.clear();

            continue;
        }

        let calories = line.parse::<i32>().expect("failed to convert line to i32");
        curr.push(calories);
    }

    total.sort();
    total.reverse();

    for i in 0..3 {
        ret += total[i];
    }

    ret
}
