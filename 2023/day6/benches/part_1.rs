use criterion::{criterion_group, criterion_main, Criterion};
use day6::part_1;

fn part_1_bench(c: &mut Criterion) {
    c.bench_function("part_1", |b| b.iter(|| part_1::run("input.txt")));
}

criterion_group!(benches, part_1_bench);
criterion_main!(benches);
