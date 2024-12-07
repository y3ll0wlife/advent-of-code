use std::fs;

fn main() {
    let p_1 = part_1();
    println!("{}", p_1);
    assert_eq!(p_1, 494)
}

fn does_include(a_1: i32, a_2: i32, b_1: i32, b_2: i32) -> bool {
    (b_1 - a_1) * (b_2 - a_2) <= 0
}

fn part_1() -> i32 {
    let input = fs::read_to_string("./input.txt").expect("failed to read file");
    let mut return_value = 0;

    for line in input.lines() {
        let mut splitter = line.split(",");

        let elf_1 = splitter.next().unwrap();
        let elf_2 = splitter.next().unwrap();

        let mut elf_1_splitter = elf_1.split("-");
        let mut elf_2_splitter = elf_2.split("-");

        let elf_1_first = elf_1_splitter.next().unwrap().parse::<i32>().unwrap();
        let elf_1_second = elf_1_splitter.next().unwrap().parse::<i32>().unwrap();

        let elf_2_first = elf_2_splitter.next().unwrap().parse::<i32>().unwrap();
        let elf_2_second = elf_2_splitter.next().unwrap().parse::<i32>().unwrap();

        if does_include(elf_1_first, elf_1_second, elf_2_first, elf_2_second) {
            return_value += 1;
        }
    }

    return_value
}
