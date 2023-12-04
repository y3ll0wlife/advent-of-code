use criterion::{criterion_group, criterion_main, Criterion};
use day4::{part_1, part_1_sets};

fn part_1_bench(c: &mut Criterion) {
    c.bench_function("part_1", |b| b.iter(|| part_1::run("input.txt")));
}
fn part_1_sets_bench(c: &mut Criterion) {
    c.bench_function("part_1_sets", |b| b.iter(|| part_1_sets::run("input.txt")));
}

criterion_group!(benches, part_1_bench, part_1_sets_bench);
criterion_main!(benches);
