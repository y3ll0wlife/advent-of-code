use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use day9::part_1::part1;

fn part_1_bench(c: &mut Criterion) {
    c.bench_function("part_1", |b| {
        b.iter(|| {
            let file = fs::read_to_string("./input.txt").expect("Failed to read file");
            let lines = file.lines().collect::<Vec<&str>>();
            part1(lines);
        })
    });
}

criterion_group!(benches, part_1_bench);
criterion_main!(benches);
