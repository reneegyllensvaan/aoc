use std::collections::{HashMap, HashSet, VecDeque};

use rustc_hash::FxHashSet;
use utils::{separate_thousands, Dir, Grid, Pos, PosUtils, SGrid, Vec2dUtils};

static INPUT_FILE: &str = "input/day16";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

pub fn part1(input: &str) -> i64 {
    let mut grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let mut seen = HashSet::new();
    traverse(&mut grid, (0, 0), Dir::Right, &mut seen);
    let seen = seen.into_iter().map(|v| v.0).collect::<HashSet<_>>();

    if cfg!(test) {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if seen.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    seen.len() as i64
}

pub fn part2(input: &str) -> i64 {
    let grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let traverse_from = |pos: Pos, dir: Dir| {
        let mut seen = HashSet::new();
        traverse(&mut grid.clone(), pos, dir, &mut seen);
        seen.into_iter().map(|v| v.0).collect::<HashSet<_>>().len() as i64
    };

    let h = grid.len();
    let w = grid[0].len();
    let mut result = 0;
    for row in 0..h {
        let a = traverse_from((row, 0), Dir::Right);
        let b = traverse_from((row, w - 1), Dir::Left);
        result = result.max(a).max(b);
    }
    for col in 0..w {
        let a = traverse_from((0, col), Dir::Down);
        let b = traverse_from((w - 1, col), Dir::Up);
        result = result.max(a).max(b);
    }

    result
}

pub fn part2_alt(input: &str) -> i64 {
    let grid: &[u8] = input.as_bytes();
    let w = grid.iter().position(|c| *c == b'\n').unwrap();
    let h = (grid.len() / w) - 1;

    let mut exits = FxHashSet::with_hasher(Default::default());
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if exits.contains(&(pos, dir.opposite())) {
            None
        } else {
            Some(traverse_unsafe(grid, h, w, pos, dir, &mut exits))
        }
    };

    let mut result = 0;
    for row in 0..h {
        if let Some(v) = traverse_from((row, 0), Dir::Right) {
            result = result.max(v);
        }
        if let Some(v) = traverse_from((row, w - 1), Dir::Left) {
            result = result.max(v);
        }
    }
    for col in 0..w {
        if let Some(v) = traverse_from((0, col), Dir::Down) {
            result = result.max(v);
        }
        if let Some(v) = traverse_from((w - 1, col), Dir::Up) {
            result = result.max(v);
        }
    }

    result
}

pub fn part2_opt(input: &str) -> i64 {
    let grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let mut outgoing_scores = HashMap::<(Pos, Dir), i64>::new();
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if let Some(v) = outgoing_scores.get(&(pos, dir.opposite())) {
            return *v;
        }
        let mut seen = HashSet::new();
        let mut exits = HashSet::new();
        traverse_track_exits(&mut grid.clone(), pos, dir, &mut seen, &mut exits);
        let out = seen.into_iter().map(|v| v.0).collect::<HashSet<_>>().len() as i64;
        for exit in exits {
            outgoing_scores.insert(exit, out);
        }
        out
    };

    let h = grid.len();
    let w = grid[0].len();
    let mut result = 0;
    for row in 0..h {
        let a = traverse_from((row, 0), Dir::Right);
        let b = traverse_from((row, w - 1), Dir::Left);
        result = result.max(a).max(b);
    }
    for col in 0..w {
        let a = traverse_from((0, col), Dir::Down);
        let b = traverse_from((w - 1, col), Dir::Up);
        result = result.max(a).max(b);
    }

    result
}

fn traverse(grid: &mut SGrid<char>, pos: Pos, dir: Dir, seen: &mut HashSet<(Pos, Dir)>) {
    if !seen.insert((pos, dir)) {
        return;
    }
    let tile = grid.at(pos);
    match tile {
        '|' if dir.horizontal() => {
            if let Some(next) = pos.go_in(Dir::Up, grid) {
                traverse(grid, next, Dir::Up, seen);
            }
            if let Some(next) = pos.go_in(Dir::Down, grid) {
                traverse(grid, next, Dir::Down, seen);
            }
        }
        '-' if dir.vertical() => {
            if let Some(next) = pos.go_in(Dir::Left, grid) {
                traverse(grid, next, Dir::Left, seen);
            }
            if let Some(next) = pos.go_in(Dir::Right, grid) {
                traverse(grid, next, Dir::Right, seen);
            }
        }
        '.' | '|' | '-' => {
            if let Some(next) = pos.go_in(dir, grid) {
                traverse(grid, next, dir, seen);
            }
        }
        '/' | '\\' => {
            let mut new_dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
            };
            if tile == '\\' {
                new_dir = new_dir.opposite();
            }
            if let Some(next) = pos.go_in(new_dir, grid) {
                traverse(grid, next, new_dir, seen);
            }
        }
        _ => panic!("unknown tile: {tile}"),
    }
}
/// Version 3.
///
/// So hashing was the expensive bit of the previous solutions. We'd do a hash table lookup for
/// each pos/dir pair.
///
/// This thing uses four vectors instead, one for each direction, and also doesn't recurse. We're
/// reusing the exit-tracking strategy from v2 - that was quite significant.
///
/// We can also squeeze another couple percent out of this approach by skipping bounds checks. This
/// is unsafe, and has no real practical utility other than go fast.
fn traverse_unsafe(
    grid: &[u8],
    h: usize,
    w: usize,
    pos: Pos,
    dir: Dir,
    exits: &mut FxHashSet<(Pos, Dir)>,
) -> i64 {
    let mut beams = VecDeque::<(Pos, Dir)>::new();
    beams.push_back((pos, dir));

    let mut seen = [
        vec![false; h * w],
        vec![false; h * w],
        vec![false; h * w],
        vec![false; h * w],
    ];
    while let Some((mut pos, mut dir)) = beams.pop_front() {
        let mut sl = unsafe { seen.get_unchecked_mut(dir as usize) };
        loop {
            let s = unsafe { sl.get_unchecked_mut(pos.0 * h + pos.1) };
            if *s {
                break;
            }
            *s = true;
            static TILT_LUT: [Dir; 4] = [Dir::Right, Dir::Up, Dir::Left, Dir::Down];
            match grid[pos.0 * (h + 1) + pos.1] {
                b'|' if dir.horizontal() => {
                    beams.push_back((pos, Dir::Up));
                    dir = Dir::Down;
                    sl = unsafe { seen.get_unchecked_mut(dir as usize) };
                }
                b'-' if dir.vertical() => {
                    beams.push_back((pos, Dir::Left));
                    dir = Dir::Right;
                    sl = unsafe { seen.get_unchecked_mut(dir as usize) };
                }
                b'/' => {
                    dir = unsafe { *TILT_LUT.get_unchecked(dir as usize) };
                    sl = unsafe { seen.get_unchecked_mut(dir as usize) };
                }
                b'\\' => {
                    dir = unsafe { *TILT_LUT.get_unchecked((dir as usize + 2) & 0b11) };
                    sl = unsafe { seen.get_unchecked_mut(dir as usize) };
                }
                _ => {}
            }

            let tar = {
                let (r, c) = pos;
                match dir {
                    Dir::Up if r == 0 => None,
                    Dir::Left if c == 0 => None,
                    Dir::Down if r >= h - 1 => None,
                    Dir::Right if c >= w - 1 => None,
                    Dir::Up => Some((r - 1, c)),
                    Dir::Down => Some((r + 1, c)),
                    Dir::Left => Some((r, c - 1)),
                    Dir::Right => Some((r, c + 1)),
                }
            };
            if let Some(new_pos) = tar {
                pos = new_pos;
            } else {
                exits.insert((pos, dir));
                break;
            }
        }
    }
    let mut result = 0;
    for ix in 0..(h * w) {
        for sl in 0..=3 {
            if unsafe { *seen.get_unchecked(sl).get_unchecked(ix) } {
                result += 1;
                break;
            }
        }
    }
    result
}
fn traverse_track_exits(
    grid: &mut SGrid<char>,
    pos: Pos,
    dir: Dir,
    seen: &mut HashSet<(Pos, Dir)>,
    exits: &mut HashSet<(Pos, Dir)>,
) {
    if !seen.insert((pos, dir)) {
        return;
    }
    let tile = grid.at(pos);
    match tile {
        '|' if dir.horizontal() => {
            if let Some(next) = pos.go_in(Dir::Up, grid) {
                traverse_track_exits(grid, next, Dir::Up, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
            if let Some(next) = pos.go_in(Dir::Down, grid) {
                traverse_track_exits(grid, next, Dir::Down, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '-' if dir.vertical() => {
            if let Some(next) = pos.go_in(Dir::Left, grid) {
                traverse_track_exits(grid, next, Dir::Left, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
            if let Some(next) = pos.go_in(Dir::Right, grid) {
                traverse_track_exits(grid, next, Dir::Right, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '.' | '|' | '-' => {
            if let Some(next) = pos.go_in(dir, grid) {
                traverse_track_exits(grid, next, dir, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '/' | '\\' => {
            let mut new_dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
            };
            if tile == '\\' {
                new_dir = new_dir.opposite();
            }
            if let Some(next) = pos.go_in(new_dir, grid) {
                traverse_track_exits(grid, next, new_dir, seen, exits);
            } else {
                exits.insert((pos, new_dir));
            }
        }
        _ => panic!("unknown tile: {tile}"),
    }
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![
        ("part1", part1),
        ("part2", part2),
        ("part2 (alt)", part2_alt),
        ("part2 (opt)", part2_opt),
    ];

    for (name, f) in &fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if !bench {
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
                separate_thousands(&(end - begin).as_micros().to_string()),
                separate_thousands(&((end - begin).as_micros() / iters).to_string())
            );
        }
    }
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 46);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 51);
    assert_eq!(part2_opt(EXAMPLE_INPUT), 51);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 7034);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 7759);
}
