use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let mut start: Option<(i64, i64)> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((row as i64, col as i64));
            }
        }
    }
    let start = start.unwrap();
    let mut costs = HashMap::<(i64, i64), i64>::new();
    costs.insert(start, 0);

    let mut conns = HashMap::<((i64, i64), (i64, i64)), i64>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i64;
            let col = col as i64;
            // up
            if let '|' | 'J' | 'L' | 'S' = c {
                *conns.entry(key((row - 1, col), (row, col))).or_default() += 1;
            }
            // down
            if let '|' | 'F' | '7' | 'S' = c {
                *conns.entry(key((row + 1, col), (row, col))).or_default() += 1;
            }
            // right
            if let '-' | 'L' | 'F' | 'S' = c {
                *conns.entry(key((row, col + 1), (row, col))).or_default() += 1;
            }
            // left
            if let '-' | 'J' | '7' | 'S' = c {
                *conns.entry(key((row, col - 1), (row, col))).or_default() += 1;
            }
        }
    }

    let conns: HashSet<((i64, i64), (i64, i64))> = conns
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .map(|(k, _)| k)
        .collect();

    let mut q = VecDeque::new();
    q.push_back(start);
    while q.len() > 0 {
        let coords = q.pop_front().unwrap();
        let (row, col) = coords;
        let cost = *costs.get(&coords).unwrap();

        let targets = [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|target| conns.contains(&key(*target, coords)))
        .filter(|target| !costs.contains_key(target))
        .collect_vec();

        for target in &targets {
            costs.insert(*target, cost + 1);
            q.push_back(*target);
        }
    }

    *costs.values().max().unwrap()
}
fn key(a: (i64, i64), b: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    (a.min(b), a.max(b))
}

pub fn part2(input: &str) -> i64 {
    let mut start: Option<(i64, i64)> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((row as i64, col as i64));
            }
        }
    }
    let start = start.unwrap();
    let mut costs = HashMap::<(i64, i64), i64>::new();
    costs.insert(start, 0);

    let mut conns = HashMap::<((i64, i64), (i64, i64)), i64>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let row = row as i64;
            let col = col as i64;
            // up
            if let '|' | 'J' | 'L' | 'S' = c {
                *conns.entry(key((row - 1, col), (row, col))).or_default() += 1;
            }
            // down
            if let '|' | 'F' | '7' | 'S' = c {
                *conns.entry(key((row + 1, col), (row, col))).or_default() += 1;
            }
            // right
            if let '-' | 'L' | 'F' | 'S' = c {
                *conns.entry(key((row, col + 1), (row, col))).or_default() += 1;
            }
            // left
            if let '-' | 'J' | '7' | 'S' = c {
                *conns.entry(key((row, col - 1), (row, col))).or_default() += 1;
            }
        }
    }

    let conns: HashSet<((i64, i64), (i64, i64))> = conns
        .into_iter()
        .filter(|(_, v)| *v >= 2)
        .map(|(k, _)| k)
        .collect();

    let mut part_of_loop = HashSet::<(i64, i64)>::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while q.len() > 0 {
        let coords = q.pop_front().unwrap();
        let (row, col) = coords;
        let cost = *costs.get(&coords).unwrap();
        part_of_loop.insert(coords.clone());

        let targets = [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|target| conns.contains(&key(*target, coords)))
        .filter(|target| !costs.contains_key(target))
        .collect_vec();

        for target in &targets {
            costs.insert(*target, cost + 1);
            q.push_back(*target);
        }
    }

    let mut space_grid = Vec::<Vec<char>>::new();
    for (row, line) in input.lines().enumerate() {
        let row = row as i64;
        if row == 0 {
            let mut space_row = vec!['1'];
            for _ in line.chars() {
                space_row.push('1');
            }
            for _ in line.chars() {
                space_row.push('1');
            }
            space_grid.push(space_row);
        }
        // first handle inside-spaces
        let mut space_row = Vec::<char>::new();
        for (col, _) in line.chars().enumerate() {
            if col == 0 {
                space_row.push('1');
            }
            let col = col as i64;
            if part_of_loop.contains(&(row as i64, col as i64)) {
                space_row.push('.');
            } else {
                space_row.push('1');
            }
            // between-spaces
            if conns.contains(&key((row, col), (row, col + 1))) {
                space_row.push('.');
            } else {
                space_row.push('1');
            }
        }
        space_grid.push(space_row);

        // run again for between spaces
        let mut space_row = Vec::<char>::new();
        for (col, _) in line.chars().enumerate() {
            if col == 0 {
                space_row.push('1');
            }
            let col = col as i64;
            if conns.contains(&key((row, col), (row + 1, col))) {
                space_row.push('.');
            } else {
                space_row.push('1');
            }
            space_row.push('1');
        }
        space_grid.push(space_row);
    }

    let mut q = VecDeque::<(i64, i64)>::new();
    q.push_back((
        space_grid.len() as i64 - 1,
        space_grid.first().unwrap().len() as i64 - 1,
    ));
    let mut seen = HashSet::<(i64, i64)>::new();
    while q.len() > 0 {
        let coords = q.pop_front().unwrap();
        let (row, col) = coords;
        space_grid[row as usize][col as usize] = '0';
        let targets = [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|(row, col)| {
            *row >= 0
                && *col >= 0
                && *row < space_grid.len() as i64
                && *col < space_grid[0].len() as i64
        })
        .filter(|v| !seen.contains(v))
        .filter(|(row, col)| space_grid[*row as usize][*col as usize] == '1')
        .collect_vec();
        for target in targets {
            q.push_back(target);
            seen.insert(target);
        }
    }

    let mut result = 0;
    for (row, line) in space_grid.into_iter().enumerate() {
        for (col, c) in line.into_iter().enumerate() {
            if c == '1' && row % 2 == 1 && col % 2 == 1 {
                result += 1;
            }
        }
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("input/day10").unwrap();

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
    let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
    assert_eq!(part1(input), 8);
}

#[test]
fn test_part2_example_1() {
    let input = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;
    assert_eq!(part2(input), 4);
}
#[test]
fn test_part2_example_2() {
    let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
    assert_eq!(part2(input), 8);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day10").unwrap();
    assert_eq!(part1(&input), 6690);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day10").unwrap();
    assert_eq!(part2(&input), 525);
}
