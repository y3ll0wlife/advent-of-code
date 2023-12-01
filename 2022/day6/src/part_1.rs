use std::{collections::HashSet, fs};

fn main() {
    let p_1 = p_1();

    println!("{}", p_1);
    assert_eq!(p_1, 1361);
}

fn p_1() -> usize {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut line: String = String::new();
    for l in input.lines() {
        line.push_str(l);
    }

    let char_vec: Vec<char> = line.chars().collect();
    let mut found = false;
    for x in 0..char_vec.len() {
        if x + 4 > char_vec.len() {
            break;
        }

        let vec_tmp = vec![
            char_vec.get(x).unwrap(),
            char_vec.get(x + 1).unwrap(),
            char_vec.get(x + 2).unwrap(),
            char_vec.get(x + 3).unwrap(),
        ];

        let v: HashSet<&char> = vec_tmp.iter().cloned().collect();
        if v.len() != 4 {
            found = true;
        } else if v.len() == 4 && found {
            return x + 4;
        }
    }
    0
}
