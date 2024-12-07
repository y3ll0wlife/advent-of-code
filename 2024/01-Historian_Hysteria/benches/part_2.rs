use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use day1::part_2::part2;

fn part_2_bench(c: &mut Criterion) {
    c.bench_function("part_2", |b| {
        b.iter(|| {
            let file = fs::read_to_string("./input.txt").expect("Failed to read file");
            let lines = file.lines().collect::<Vec<&str>>();
            part2(lines);
        })
    });
}

criterion_group!(benches, part_2_bench);
criterion_main!(benches);
