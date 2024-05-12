use crate::part_1;

#[test]
fn part_1_test() {
    let test_part_1 = part_1::run("testinput.txt");
    println!("part 1 test value: {}", test_part_1);
    assert_eq!(test_part_1, 4);
}

#[test]
fn part_1() {
    let part_1 = part_1::run("input.txt");
    println!("part 1 value: {}", part_1);
    assert_eq!(part_1, 2592);
}
