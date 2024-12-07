use std::fs;

pub fn run(file_name: &str) -> usize {
    let input = fs::read_to_string(file_name).expect("failed to read file");
    let mut total: Vec<usize> = vec![];

    for line in input.lines() {
        let vals: Vec<usize> = line
            .split("x")
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        let length = vals[0];
        let width = vals[1];
        let height = vals[2];

        let side_1 = length + width;
        let side_2 = width + height;
        let side_3 = height + length;

        let mut smallest = side_1;
        if smallest > side_2 {
            smallest = side_2
        }
        if smallest > side_3 {
            smallest = side_3;
        }

        total.push(2 * smallest);
        total.push(length * width * height);
    }

    total.iter().sum()
}
