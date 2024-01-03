use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let serieses = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();

    serieses.into_iter().map(|v| predict(&v)).sum()
}
fn predict(series: &[i64]) -> i64 {
    let changes: Vec<_> = series
        .into_iter()
        .zip(series.into_iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    let delta = if changes.iter().all(|v| *v == 0) {
        0
    } else {
        predict(&changes)
    };
    series.last().unwrap() + delta
}

pub fn part2(input: &str) -> i64 {
    let serieses = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();

    serieses.into_iter().map(|v| predict_backwards(&v)).sum()
}
fn predict_backwards(series: &[i64]) -> i64 {
    let changes: Vec<_> = series
        .into_iter()
        .zip(series.into_iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    let delta = if changes.iter().all(|v| *v == 0) {
        0
    } else {
        predict_backwards(&changes)
    };
    series.first().unwrap() - delta
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string("input/day09").unwrap();

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 2] = [("part1", part1), ("part2", part2)];

    for (name, f) in fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
        for (name, f) in fns {
            let begin = std::time::Instant::now();
            for _ in 0..iters {
                f(&input);
            }
            let end = std::time::Instant::now();
            println!(
                "  {} {} in: {}us ({}us/iter)",
                iters,
                name,
                (end - begin).as_micros(),
                (end - begin).as_micros() / iters
            );
        }
    }
}

#[test]
fn test_part1_example() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    assert_eq!(part1(input), 114);
}

#[test]
fn test_part2_example() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    assert_eq!(part2(input), 2);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day09").unwrap();
    assert_eq!(part1(&input), 1955513104);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day09").unwrap();
    assert_eq!(part2(&input), 1131);
}
