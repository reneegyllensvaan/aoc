use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use utils::{Grid, Pos, PosUtils, Vec2dUtils};

static INPUT_FILE: &str = "input/day21";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

pub fn part1_example(input: &str) -> i64 {
    part1(input, 6)
}
pub fn part1_real(input: &str) -> i64 {
    part1(input, 64)
}
pub fn part1(input: &str, num_steps: i64) -> i64 {
    let mut grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();
    let start: Pos = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| Some((row, line.iter().find_position(|c| **c == 'S')?.0)))
        .next()
        .unwrap();
    grid[start.0][start.1] = '.';

    let mut ends = HashSet::<Pos>::new();
    let mut seen = HashMap::<Pos, i64>::new();
    let mut q = VecDeque::<(Pos, i64)>::new();
    q.push_back((start, 0));
    while let Some((pos, steps)) = q.pop_front() {
        if !seen.contains_key(&pos) && steps <= num_steps {
            seen.insert(pos, steps);
            if steps == num_steps {
                ends.insert(pos);
            } else {
                for neigh in pos.neighbors_in(&grid) {
                    if grid.at(neigh) == '.' {
                        q.push_back((neigh, steps + 1));
                    }
                }
            }
        }
    }

    seen.into_iter().filter(|v| v.1 % 2 == 0).count() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();
    let start: Pos = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| Some((row, line.iter().find_position(|c| **c == 'S')?.0)))
        .next()
        .unwrap();
    grid[start.0][start.1] = '.';
    let h = grid.len();
    let mut ends = HashSet::<Pos>::new();
    let mut seen = HashMap::<Pos, i64>::new();
    let mut q = VecDeque::<(Pos, i64)>::new();
    q.push_back((start, 0));
    while let Some((pos, steps)) = q.pop_front() {
        if !seen.contains_key(&pos) {
            seen.insert(pos, steps);
            ends.insert(pos);
            for neigh in pos.neighbors_in(&grid) {
                if grid.at(neigh) == '.' {
                    q.push_back((neigh, steps + 1));
                }
            }
        }
    }

    // based off of this angel's breakdown: https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    let even_corners = seen.values().filter(|v| **v % 2 == 0 && **v > 65).count();
    let odd_corners = seen.values().filter(|v| **v % 2 == 1 && **v > 65).count();
    let n = ((26501365 - (h / 2)) / h) as usize;
    assert_eq!(n, 202300);
    let even_full = seen.values().filter(|v| **v % 2 == 0).count();
    let odd_full = seen.values().filter(|v| **v % 2 == 1).count();
    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;
    p2 as i64
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1_real), ("part2", part2)];

    for (name, f) in &fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
        for (name, f) in &fns {
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
    assert_eq!(part1_example(EXAMPLE_INPUT), 16);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1_real(&input), 3697);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 608152828731262);
}
