use itertools::Itertools;

fn solve(input: &str, factor: i64) -> i64 {
    let mut rows = 0i64;
    let mut cols = 0i64;
    let mut galaxies = Vec::<(i64, i64)>::new();
    for (row, line) in input.lines().enumerate() {
        cols = line.len() as i64;
        rows += 1;

        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((row as i64, col as i64));
            }
        }
    }

    let expanded_rows = (0..rows)
        .filter(|row| !galaxies.iter().any(|(r, _)| r == row))
        .collect_vec();
    let expanded_cols = (0..cols)
        .filter(|col| !galaxies.iter().any(|(_, c)| c == col))
        .collect_vec();

    let mut result = 0;
    for a in &galaxies {
        for b in &galaxies {
            // skipping a>b ensures we only do each pair once
            if a == b || a > b {
                continue;
            }
            let y0 = a.0.min(b.0);
            let y1 = a.0.max(b.0);
            let y_ex: i64 = expanded_rows
                .iter()
                .filter(|row| **row > y0 && **row < y1)
                .map(|_| factor - 1)
                .sum();

            let x0 = a.1.min(b.1);
            let x1 = a.1.max(b.1);
            let x_ex: i64 = expanded_cols
                .iter()
                .filter(|col| **col > x0 && **col < x1)
                .map(|_| factor - 1)
                .sum();

            result += y1 - y0 + y_ex + x1 - x0 + x_ex;
        }
    }

    result
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 1000000)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day11").unwrap();

    let iters = 100;

    let fns: [(&'static str, fn(&str) -> i64); 2] = [("part1", part1), ("part2", part2)];

    for (name, f) in fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
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

#[test]
fn test_part1_example() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    assert_eq!(part1(input), 374);
}

#[test]
fn test_part2_example() {
    let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    assert_eq!(part2(input), 82000210);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day11").unwrap();
    assert_eq!(part1(&input), 9312968);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day11").unwrap();
    assert_eq!(part2(&input), 597714117556);
}
