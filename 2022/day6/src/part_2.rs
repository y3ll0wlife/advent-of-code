use std::{collections::HashSet, fs};

fn main() {
    let p_2 = p_2();

    println!("{}", p_2);
    assert_eq!(p_2, 3263);
}

fn p_2() -> usize {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let mut line: String = String::new();
    for l in input.lines() {
        line.push_str(l);
    }

    let char_vec: Vec<char> = line.chars().collect();
    let mut found = false;

    let amt = 14;

    for x in 0..char_vec.len() {
        if x + amt > char_vec.len() {
            break;
        }
        let mut vec_tmp = vec![];
        for amount in 0..amt {
            vec_tmp.push(char_vec.get(x + amount).unwrap())
        }

        let v: HashSet<&char> = vec_tmp.iter().cloned().collect();
        if v.len() != amt {
            found = true;
        } else if v.len() == amt && found {
            return x + amt;
        }
    }
    0
}
