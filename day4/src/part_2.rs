use std::fs;

fn main() {
    let p_2 = part_2();
    println!("{}", p_2);
    assert_eq!(p_2, 833)
}

fn does_include(a_1: i32, a_2: i32, b_1: i32, b_2: i32) -> bool {
    a_1.max(b_1) <= b_2.min(a_2)
}

fn part_2() -> i32 {
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
