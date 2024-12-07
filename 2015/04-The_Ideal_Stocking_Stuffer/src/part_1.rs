use core::str;

pub fn part1(input: &str) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let string = format!("{:x}", digest);

        if string.starts_with("00000") {
            break;
        }

        i += 1;
    }

    i
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }

    #[test]
    fn part_1() {
        assert_eq!(part1("iwrupvqb"), 346386);
    }
}
