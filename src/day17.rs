use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::utils::{Dir, Grid, Pos, PosUtils, Vec2dUtils};

static INPUT_FILE: &str = "input/day17";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

pub fn part1(input: &str) -> i64 {
    let grid: Grid<u8> = input
        .lines()
        .map(|l| l.as_bytes().into_iter().map(|c| c - b'0').collect())
        .collect();

    let h = grid.len();
    let w = grid[0].len();
    let final_pos = (h - 1, w - 1);

    let mut seen = HashSet::<(Pos, Dir, usize)>::new();
    let mut q = BinaryHeap::<Reverse<(i64, Pos, Dir, usize)>>::new();
    q.push(Reverse((0, (0, 0), Dir::Right, 0)));
    q.push(Reverse((0, (0, 0), Dir::Down, 0)));

    while q.len() > 0 {
        let Reverse((loss, pos, dir, cons)) = q.pop().unwrap();

        if pos == final_pos {
            return loss;
        }

        if !seen.insert((pos, dir, cons)) {
            continue;
        }

        // first, try moving ahead.
        if cons <= 2 {
            if let Some(m) = pos.go_in(dir, &grid) {
                q.push(Reverse((loss + grid.at(m) as i64, m, dir, cons + 1)));
            }
        }
        if let Some(m) = pos.go_in(dir.turn_left(), &grid) {
            q.push(Reverse((loss + grid.at(m) as i64, m, dir.turn_left(), 1)));
        }
        if let Some(m) = pos.go_in(dir.turn_right(), &grid) {
            q.push(Reverse((loss + grid.at(m) as i64, m, dir.turn_right(), 1)));
        }
    }

    seen.into_iter()
        .filter(|v| v.0 == final_pos)
        .map(|v| v.1)
        .min()
        .unwrap() as i64
}

pub fn part2(input: &str) -> i64 {
    let grid: Grid<u8> = input
        .lines()
        .map(|l| l.as_bytes().into_iter().map(|c| c - b'0').collect())
        .collect();

    let h = grid.len();
    let w = grid[0].len();
    let final_pos = (h - 1, w - 1);

    let mut seen = HashSet::<(Pos, Dir, usize)>::new();
    let mut q = BinaryHeap::<Reverse<(i64, Pos, Dir, usize)>>::new();
    q.push(Reverse((0, (0, 0), Dir::Right, 0)));
    q.push(Reverse((0, (0, 0), Dir::Down, 0)));

    while q.len() > 0 {
        let Reverse((loss, pos, dir, cons)) = q.pop().unwrap();

        if pos == final_pos {
            if cons >= 4 {
                return loss;
            }
        }

        if !seen.insert((pos, dir, cons)) {
            continue;
        }

        // first, try moving ahead.
        if cons < 10 {
            if let Some(m) = pos.go_in(dir, &grid) {
                q.push(Reverse((loss + grid.at(m) as i64, m, dir, cons + 1)));
            }
        }
        // only if we've got enough consecutive movements should we be allowed to turn same
        // direction
        if cons >= 4 {
            if let Some(m) = pos.go_in(dir.turn_left(), &grid) {
                q.push(Reverse((loss + grid.at(m) as i64, m, dir.turn_left(), 1)));
            }
            if let Some(m) = pos.go_in(dir.turn_right(), &grid) {
                q.push(Reverse((loss + grid.at(m) as i64, m, dir.turn_right(), 1)));
            }
        }
    }

    panic!("can this happen?")
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
    assert_eq!(part1(EXAMPLE_INPUT), 102);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 94);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 886);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 1055);
}
