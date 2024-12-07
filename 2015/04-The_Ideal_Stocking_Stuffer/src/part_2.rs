use core::str;

pub fn part2(input: &str) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let string = format!("{:x}", digest);

        if string.starts_with("000000") {
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
    fn part_2() {
        assert_eq!(part2("iwrupvqb"), 9958218);
    }
}
