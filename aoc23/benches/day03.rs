use aoc23::day03::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("input/day03").unwrap();
    c.bench_function("part1", |b| b.iter(|| part1(&input)));
    c.bench_function("part2", |b| b.iter(|| part2(&input)));
    c.bench_function("part1 (btree)", |b| b.iter(|| part1_btree(&input)));
    c.bench_function("part1 (hash)", |b| b.iter(|| part1_hash(&input)));
    c.bench_function("part2 (regex)", |b| b.iter(|| part2_regex(&input)));
    c.bench_function("part2 (no regex)", |b| b.iter(|| part2_no_regex(&input)));
    c.bench_function("part2 (btree)", |b| b.iter(|| part2_btree(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
