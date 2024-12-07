use std::fs;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

fn main() {
    let p_2 = p_2();

    println!("{}", p_2);
    assert_eq!(p_2, "QNDWLMGNS");
}

fn p_2() -> String {
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
                .insert(i as usize, removed);
            println!("{:?}", crates);
        }
    }

    let mut answer: String = String::new();

    for c in crates {
        answer.push_str(c[0]);
    }

    answer
}
