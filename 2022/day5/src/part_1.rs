use std::fs;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let p_1 = p_1();

    println!("{}", p_1);
    assert_eq!(p_1, "CNSZFDVLJ");
}

fn p_1() -> String {
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    //let mut crates = vec![vec!["N", "Z"], vec!["D", "C", "M"], vec!["P"]];
    let mut crates = vec![
        vec!["N", "V", "C", "S"],
        vec!["S", "N", "H", "J", "M", "Z"],
        vec!["D", "N", "J", "G", "T", "C", "M"],
        vec!["M", "R", "W", "J", "F", "D", "T"],
        vec!["H", "F", "P"],
        vec!["J", "H", "Z", "T", "C"],
        vec!["Z", "L", "S", "F", "Q", "R", "P", "D"],
        vec!["W", "P", "F", "D", "H", "L", "S", "C"],
        vec!["Z", "G", "N", "F", "P", "M", "S", "D"],
    ];

    /*
                            [Z] [W] [Z]
            [D] [M]         [L] [P] [G]
        [S] [N] [R]         [S] [F] [N]
        [N] [J] [W]     [J] [F] [D] [F]
    [N] [H] [G] [J]     [H] [Q] [H] [P]
    [V] [J] [T] [F] [H] [Z] [R] [L] [M]
    [C] [M] [C] [D] [F] [T] [P] [S] [S]
    [S] [Z] [M] [T] [P] [C] [D] [C] [D]
     1   2   3   4   5   6   7   8   9

    */

    println!("{:?}", crates);

    for line in input.lines() {
        let (_m, amount, _f, from_pile, _t, to_pile) = scan!(
            line,
            char::is_whitespace,
            String,
            i32,
            String,
            usize,
            String,
            usize
        );

        println!(
            "moving {} from pile {} to pile {}",
            amount.unwrap(),
            from_pile.unwrap(),
            to_pile.unwrap()
        );

        let mut removed: &str;

        for i in 0..amount.unwrap() {
            {
                let current_pile = crates
                    .get_mut(from_pile.unwrap() - 1)
                    .expect("failed to find the 'from_pile'");
                removed = current_pile.remove(0);
            }
            println!("- move {} crate ({})", i, removed);
            crates
                .get_mut(to_pile.unwrap() - 1)
                .unwrap()
                .insert(0, removed);
            println!("{:?}", crates);
        }
    }

    let mut answer: String = String::new();

    for c in crates {
        answer.push_str(c[0]);
    }

    answer
}
