use criterion::{criterion_group, criterion_main, Criterion};
use day5::part_2;

fn part_2_bench(c: &mut Criterion) {
    c.bench_function("part_2", |b| b.iter(|| part_2::run("input.txt")));
}

criterion_group!(benches, part_2_bench);
criterion_main!(benches);
