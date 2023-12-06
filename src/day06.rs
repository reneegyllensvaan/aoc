use itertools::Itertools;

/// Brute-force solution.
///
/// Iterates over all the different charges.
fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .collect::<Vec<_>>();

    let mut result: i64 = 1;
    for (time, winner) in times.into_iter().zip(distances) {
        let mut winning_charges = 0;
        for charge in 0..=time {
            let distance = charge * (time - charge);
            if distance > winner {
                winning_charges += 1;
            }
        }
        if winning_charges >= 0 {
            result *= winning_charges;
        }
    }

    result
}

#[allow(dead_code)]
fn part2_math(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<f64>()
        .unwrap();
    let winner = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<f64>()
        .unwrap();

    ((time + (time * time - 4.0 * winner).sqrt()) / 2.0).floor() as i64
        - ((time - (time * time - 4.0 * winner).sqrt()) / 2.0).ceil() as i64
        + 1
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<i64>()
        .unwrap();
    let winner = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<i64>()
        .unwrap();

    let mut result: i64 = 1;
    let mut winning_charges = 0;
    for charge in 0..=time {
        let distance = charge * (time - charge);
        if distance > winner {
            winning_charges += 1;
        }
    }
    if winning_charges >= 0 {
        result *= winning_charges;
    }

    result
}

pub fn main() {
    let input = std::fs::read_to_string("input/day06").unwrap();

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 3] = [
        ("part1", part1),
        ("part2", part2),
        ("part2 (math)", part2_math),
    ];

    for (name, f) in fns {
        println!("{name}: {}", f(&input));
    }

    for (name, f) in fns {
        let begin = std::time::Instant::now();
        for _ in 0..iters {
            f(&input);
        }
        let end = std::time::Instant::now();
        println!(
            "{} {} in: {}ns ({}ns/iter)",
            iters,
            name,
            (end - begin).as_nanos(),
            (end - begin).as_nanos() / iters
        );
    }
}

#[test]
fn test_part1_example() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(part1(input), 288);
}

#[test]
fn test_part2_example() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    assert_eq!(part2(input), 71503);
    assert_eq!(part2_math(input), 71503);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day06").unwrap();
    assert_eq!(part1(&input), 2065338);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day06").unwrap();
    assert_eq!(part2(&input), 34934171);
    assert_eq!(part2_math(&input), 34934171);
}
