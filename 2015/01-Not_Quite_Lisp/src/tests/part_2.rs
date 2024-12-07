use crate::part_2;

#[test]
fn part_2_test() {
    assert_eq!(part_2::run("testinput_6.txt"), 1);
    assert_eq!(part_2::run("testinput_7.txt"), 5);
}

#[test]
fn part_2() {
    let part_2 = part_2::run("input.txt");
    println!("part 2 value: {}", part_2);
    assert_eq!(part_2, 1797);
}
