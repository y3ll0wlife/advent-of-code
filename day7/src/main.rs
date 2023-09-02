use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    println!("##### Part 1 #####");

    let test_p_1 = p_1("testinput.txt");
    println!("test value: {}", test_p_1);
    assert_eq!(test_p_1, 95437);

    let p_1 = p_1("input.txt");
    println!("real value: {}", p_1);
    assert_eq!(p_1, 1453349);

    println!("##### Part 2 #####");

    let p_2 = p_2("input.txt");
    println!("real value: {}", p_2);
    assert_eq!(p_2, 2948823);
}

fn p_1(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut current_dirs: Vec<&str> = vec![];

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let line_parts: Vec<&str> = line.split_whitespace().collect();
        match line_parts[..] {
            ["$", "cd", ".."] => {
                current_dirs.pop().expect("failed to pop empty vector");
            }
            ["$", "cd", name] => {
                current_dirs.push(name);
            }
            [size, _name] => {
                let size: usize = size.parse().expect("failed to parse size");
                for i in 0..current_dirs.len() {
                    let path = PathBuf::from_iter(&current_dirs[..=i]);
                    *dir_sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }

    dir_sizes.into_values().filter(|f| *f <= 100_000).sum()
}

fn p_2(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");

    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut current_dirs: Vec<&str> = vec![];

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let line_parts: Vec<&str> = line.split_whitespace().collect();
        match line_parts[..] {
            ["$", "cd", ".."] => {
                current_dirs.pop().expect("failed to pop empty vector");
            }
            ["$", "cd", name] => {
                current_dirs.push(name);
            }
            [size, _name] => {
                let size: usize = size.parse().expect("failed to parse size");
                for i in 0..current_dirs.len() {
                    let path = PathBuf::from_iter(&current_dirs[..=i]);
                    *dir_sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }

    let total_size = 70_000_000;
    let needed_size = 30_000_000;

    let root = dir_sizes.get(&PathBuf::from("/")).unwrap();
    let available = total_size - root;

    dir_sizes
        .into_values()
        .filter(|size| available + size >= needed_size)
        .min()
        .expect("failed to get min value")
}
