#[derive(Debug)]
struct Disk {
    id: Option<usize>,
    size: usize,
}

fn get_str_disk(disk: Vec<Disk>) -> Vec<String> {
    let mut str = vec![];
    for d in disk {
        if d.id.is_none() {
            for _ in 1..=d.size {
                str.push(String::from("."));
            }
            continue;
        }

        for _ in 1..=d.size {
            str.push(d.id.unwrap().to_string().parse().unwrap());
        }
    }

    str
}

pub fn part1(input: Vec<&str>) -> usize {
    let input = input.first().unwrap().to_string();
    let mut id = 0;
    let mut is_file = true;
    let mut disk: Vec<Disk> = vec![];

    for c in input.chars() {
        let num = c.to_string().parse::<usize>().unwrap();

        if is_file {
            disk.push(Disk {
                id: Some(id),
                size: num,
            });
            id += 1;
        } else {
            disk.push(Disk {
                id: None,
                size: num,
            })
        }
        is_file = !is_file;
    }

    let mut disk_string = get_str_disk(disk);

    loop {
        let num1 = disk_string.iter().position(|x| x == ".").unwrap();
        let num2 = disk_string.len() - disk_string.iter().rev().position(|x| x != ".").unwrap() - 1;

        let clean_disk = &disk_string[num1 + 1..disk_string.len()]
            .iter()
            .all(|c| c == ".");
        if *clean_disk {
            break;
        }

        disk_string.swap(num1, num2);
    }

    let mut total = 0;
    for (i, v) in disk_string
        .iter()
        .filter(|f| f != &".")
        .map(|f| f.to_string().parse::<usize>().unwrap())
        .enumerate()
    {
        total += i * v;
    }

    total
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_test() {
        assert_eq!(part1(vec!["2333133121414131402"]), 1928);
    }

    //#[test]
    fn part_1() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part1(lines), 6307275788409);
    }
}
