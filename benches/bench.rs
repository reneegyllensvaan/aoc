use aoc23::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    bench_day01(c);
    bench_day02(c);
    bench_day03(c);
    bench_day04(c);
    bench_day05(c);
    bench_day06(c);
}
fn bench_day01(c: &mut Criterion) {
    let mut group = c.benchmark_group("day01");
    let input = std::fs::read_to_string("input/day01").unwrap();
    group.bench_function("part1", |b| b.iter(|| day01::part1(&input)));
    group.bench_function("part2 (regex)", |b| b.iter(|| day01::part2_regex(&input)));
    group.bench_function("part2 (no regex)", |b| {
        b.iter(|| day01::part2_no_regex(&input))
    });
    group.bench_function("part2 (no regex, bidir)", |b| {
        b.iter(|| day01::part2_no_regex_bidir(&input))
    });
    group.bench_function("part2 (no regex, bidir, add directly)", |b| {
        b.iter(|| day01::part2_no_regex_bidir_add_directly(&input))
    });
    group.bench_function("part2 (no regex, bidir, add directly, byte lines)", |b| {
        b.iter(|| day01::part2_no_regex_bidir_add_directly_byte_lines(&input))
    });
}

fn bench_day02(c: &mut Criterion) {
    let mut group = c.benchmark_group("day02");
    let input = std::fs::read_to_string("input/day02").unwrap();
    group.bench_function("part1", |b| b.iter(|| day02::part1(&input)));
    group.bench_function("part2", |b| b.iter(|| day02::part2(&input)));
}

fn bench_day03(c: &mut Criterion) {
    let mut group = c.benchmark_group("day03");
    let input = std::fs::read_to_string("input/day03").unwrap();
    group.bench_function("part1", |b| b.iter(|| day03::part1(&input)));
    group.bench_function("part2", |b| b.iter(|| day03::part2(&input)));
    group.bench_function("part1 (btree)", |b| b.iter(|| day03::part1_btree(&input)));
    group.bench_function("part1 (hash)", |b| b.iter(|| day03::part1_hash(&input)));
    group.bench_function("part2 (regex)", |b| b.iter(|| day03::part2_regex(&input)));
    group.bench_function("part2 (no regex)", |b| {
        b.iter(|| day03::part2_no_regex(&input))
    });
    group.bench_function("part2 (btree)", |b| b.iter(|| day03::part2_btree(&input)));
}

fn bench_day04(c: &mut Criterion) {
    let mut group = c.benchmark_group("day04");
    let input = std::fs::read_to_string("input/day04").unwrap();
    group.bench_function("part1", |b| b.iter(|| day04::part1(&input)));
    group.bench_function("part2", |b| b.iter(|| day04::part2(&input)));
    group.bench_function("part1 (hash set)", |b| {
        b.iter(|| day04::part2_hash_set(&input))
    });
    group.bench_function("part1 (btree)", |b| b.iter(|| day04::part2_btree(&input)));
}

fn bench_day05(c: &mut Criterion) {
    let mut group = c.benchmark_group("day05");
    let input = std::fs::read_to_string("input/day05").unwrap();
    group.bench_function("part1", |b| b.iter(|| day05::part1(&input)));
    group.bench_function("part2", |b| b.iter(|| day05::part2(&input)));
}

fn bench_day06(c: &mut Criterion) {
    let mut group = c.benchmark_group("day06");
    let input = std::fs::read_to_string("input/day06").unwrap();
    group.bench_function("part1", |b| b.iter(|| day06::part1(&input)));
    group.bench_function("part2", |b| b.iter(|| day06::part2(&input)));
    group.bench_function("part2 (math)", |b| b.iter(|| day06::part2_math(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
