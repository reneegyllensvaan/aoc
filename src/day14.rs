use std::collections::HashMap;

use crate::utils::Vec2dUtils;

static INPUT_FILE: &str = "input/day14";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

pub fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // first, tilt north
    let mut grid = grid.transpose();
    for col in grid.iter_mut() {
        let mut changed = true;
        while changed {
            changed = false;
            for ix in 0..col.len() {
                let v = col[ix];
                if v == '.' {
                    let t = ix + 1;
                    if t < col.len() && col[t] == 'O' {
                        col[ix] = 'O';
                        col[t] = '.';
                        changed = true;
                    }
                }
            }
        }
    }
    let grid = grid.transpose();

    // then, calculate all the weights
    let mut result = 0;
    for (row, score) in grid.iter().rev().zip(1..) {
        for col in row {
            if *col == 'O' {
                result += score;
            }
        }
    }

    result
}

pub fn part2(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut seen = HashMap::<Vec<Vec<char>>, usize>::new();

    let mut cycle_ix: Option<(usize, usize)> = None;
    for ix in 0..1_000_000_000 {
        if let Some(start) = seen.get(&grid) {
            cycle_ix = Some((*start, ix));
            break;
        }
        seen.insert(grid.clone(), ix);
        grid = spin_cycle(grid);
    }

    // if we found a cycle, we need to run some logic to spin up from the cycle
    if let Some((cycle_start, cycle_end)) = cycle_ix {
        let iters = (1_000_000_000 - cycle_start) % (cycle_end - cycle_start);
        for _ in 0..iters {
            grid = spin_cycle(grid);
        }
    }

    grid.transpose();

    // then, calculate all the weights
    let mut result = 0;
    for (row, score) in grid.iter().rev().zip(1..) {
        for col in row {
            if *col == 'O' {
                result += score;
            }
        }
    }

    result
}

fn spin_cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        grid = rot90(&tilt_north(&grid));
    }
    grid
}
fn rot90(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::<Vec<char>>::new();
    let h = grid.len();
    let w = grid[0].len();
    for c in 0..w {
        let mut col = Vec::<char>::with_capacity(w);
        for r in (0..h).rev() {
            col.push(grid[r][c]);
        }
        result.push(col);
    }
    result
}
fn tilt_north(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut grid = grid.transpose();
    for col in grid.iter_mut() {
        let mut changed = true;
        while changed {
            changed = false;
            // TODO: this thing runs quadratically. you can group the items by type and count, and
            // then merge the groups in one pass. might be quicker.
            for ix in 0..col.len() {
                let v = col[ix];
                if v == '.' {
                    let t = ix + 1;
                    if t < col.len() && col[t] == 'O' {
                        col[ix] = 'O';
                        col[t] = '.';
                        changed = true;
                    }
                }
            }
        }
    }
    grid.transpose()
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1), ("part2", part2)];

    for (name, f) in &fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
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

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 136);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 64);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 113525);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 101292);
}
