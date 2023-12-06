use crate::part_1;

#[test]
fn part_1_test() {
    assert_eq!(part_1::run("testinput_1.txt"), 0);
    assert_eq!(part_1::run("testinput_2.txt"), 3);
    assert_eq!(part_1::run("testinput_3.txt"), 3);
    assert_eq!(part_1::run("testinput_4.txt"), -1);
    assert_eq!(part_1::run("testinput_5.txt"), -3);
}

#[test]
fn part_1() {
    let part_1 = part_1::run("input.txt");
    println!("part 1 value: {}", part_1);
    assert_eq!(part_1, 280);
}
