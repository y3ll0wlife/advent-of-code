use criterion::{criterion_group, criterion_main, Criterion};
use day4::{part_2, part_2_sets};

fn part_2_bench(c: &mut Criterion) {
    c.bench_function("part_2", |b| b.iter(|| part_2::run("input.txt")));
}
fn part_2_bench_sets(c: &mut Criterion) {
    c.bench_function("part_2_sets", |b| b.iter(|| part_2_sets::run("input.txt")));
}

criterion_group!(benches, part_2_bench, part_2_bench_sets);
criterion_main!(benches);
