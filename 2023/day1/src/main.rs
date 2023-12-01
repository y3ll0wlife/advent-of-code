mod part_1;
mod part_2;

fn main() {
    let test_part_1 = part_1::run("testinput.txt");
    println!("part 1 test value: {}", test_part_1);
    assert_eq!(test_part_1, 142);

    let part_1 = part_1::run("input.txt");
    println!("part 1 value: {}", part_1);
    assert_eq!(part_1, 54304);

    println!("---------------------");

    let test_part_2 = part_2::run("testinput_2.txt");
    println!("part 2 test value: {}", test_part_2);
    assert_eq!(test_part_2, 281);

    let part_2 = part_2::run("input.txt");
    println!("part 2 value: {}", part_2);
    assert_eq!(part_2, 54418);
}
