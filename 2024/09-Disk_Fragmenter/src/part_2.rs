#[derive(Debug, Clone)]
struct Disk {
    id: String,
    size: usize,
}

pub fn part2(input: Vec<&str>) -> usize {
    let input = input.first().unwrap().to_string();
    let mut id = 0;
    let mut is_file = true;
    let mut disk: Vec<Disk> = vec![];

    for c in input.chars() {
        let num = c.to_string().parse::<usize>().unwrap();

        if is_file {
            disk.push(Disk {
                id: id.to_string(),
                size: num,
            });
            id += 1;
        } else {
            disk.push(Disk {
                id: String::from("."),
                size: num,
            })
        }
        is_file = !is_file;
    }

    let largest_disk = disk
        .iter()
        .filter(|c| c.id != ".")
        .max_by(|a, b| {
            a.id.parse::<usize>()
                .unwrap()
                .cmp(&b.id.parse::<usize>().unwrap())
        })
        .unwrap();
    let disk_id = largest_disk.id.clone().parse::<usize>().unwrap();

    for d_id in (0..=disk_id).rev() {
        let a = disk.iter().position(|x| x.id == d_id.to_string()).unwrap();
        let disk_a = disk.get(a).unwrap();

        let b = disk
            .iter()
            .position(|x| x.id == "." && disk_a.size <= x.size);

        if b.is_none() {
            continue;
        }

        let b = b.unwrap();

        if a < b {
            continue;
        }

        let disk_b = disk.get(b).unwrap();

        let disk_b_size = disk_b.size;
        let new_disk_b_size = disk_b.size - disk_a.size;

        if new_disk_b_size > 0 {
            disk.insert(
                b + 1,
                Disk {
                    id: String::from("."),
                    size: new_disk_b_size,
                },
            );
            let disk_b_mut = disk.get_mut(b).unwrap();
            disk_b_mut.size = disk_b_size - new_disk_b_size;
            disk.swap(a + 1, b);
        } else {
            disk.swap(b, a);
        }
    }

    let mut str = vec![];
    for d in disk {
        for _ in 1..=d.size {
            str.push(d.id.clone());
        }
    }

    let mut total = 0;

    for (i, v) in str
        .iter()
        .map(|f| f.to_string().parse::<usize>())
        .enumerate()
    {
        if v.is_err() {
            continue;
        }

        total += i * v.unwrap();
    }

    total
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_2_test() {
        assert_eq!(part2(vec!["2333133121414131402"]), 2858);
    }

    #[test]
    fn part_2() {
        let file = fs::read_to_string("./input.txt").expect("failed to read file");
        let lines = file.lines().collect::<Vec<&str>>();
        assert_eq!(part2(lines), 6327174563252);
    }
}
