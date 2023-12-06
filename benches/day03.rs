use aoc23::day03::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("input/day03").unwrap();
    c.bench_function("part1", |b| b.iter(|| part1(&input)));
    c.bench_function("part2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
