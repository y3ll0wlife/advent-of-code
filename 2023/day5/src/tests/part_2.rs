use crate::part_2;

//#[test]
fn part_2_test() {
    let test_part_2 = part_2::run("testinput_2.txt");
    println!("part 2 test value: {}", test_part_2);
    assert_eq!(test_part_2, 0);
}

//#[test]
fn part_2() {
    let part_2 = part_2::run("input.txt");
    println!("part 2 value: {}", part_2);
    assert_eq!(part_2, 0);
}
