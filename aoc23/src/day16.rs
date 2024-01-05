use std::collections::{HashMap, HashSet, VecDeque};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
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

/// Version 1.
///
/// This was a fun problem! This one just recursively walks each beam, splitting into two, and
/// maintaining a hash set cache of pos/dir states.
///
/// Looks like most of the time is spent hashing pos/dir entries, so that'd be prime to optimize.
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

/// Version 2.
///
/// So since beams are undirected (they'll happen the same way in one direction as in the
/// opposite), if we track how we exit the grid in one beam, we don't need to check that start
/// position in the opposite direction.
pub fn part2_opt(input: &str) -> i64 {
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

/// Version 3.
///
/// This one gathers a couple different optimizations:
/// - Don't parse the grid into a grid of chars; use a big list of u8. More compact in memory.
/// - Instead of tracking seen states in a pos/dir tuple hash set, make a big empty list for each
///   direction. We're wasting a lot of space, maybe we could use bitmaps?
/// - Don't recurse. This is mostly because profiles get really annoying to read recursively, but
///   I'm sure it doesn't help performance to push the stack that high either.
/// - This thing also reuses beams as they pass through reflections. Tilts just change the
///   direction of the current beam, splitters change the direction of the current beam and push a
///   new one in the other direction
/// - Reuse the exit-tracking trick from v2.
pub fn part2_lincache(input: &str) -> i64 {
    fn traverse_lincache(
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
            loop {
                let s = &mut seen[dir as usize][pos.0 * h + pos.1];
                if *s {
                    break;
                }
                *s = true;
                static TILT_LUT: [Dir; 4] = [Dir::Right, Dir::Up, Dir::Left, Dir::Down];
                match grid[pos.0 * (h + 1) + pos.1] {
                    b'|' if dir.horizontal() => {
                        beams.push_back((pos, Dir::Up));
                        dir = Dir::Down;
                    }
                    b'-' if dir.vertical() => {
                        beams.push_back((pos, Dir::Left));
                        dir = Dir::Right;
                    }
                    b'/' => {
                        dir = TILT_LUT[dir as usize];
                    }
                    b'\\' => {
                        dir = TILT_LUT[(dir as usize + 2) & 0b11];
                    }
                    _ => {}
                }

                let tar = match dir {
                    Dir::Up if pos.0 == 0 => None,
                    Dir::Left if pos.1 == 0 => None,
                    Dir::Down if pos.0 >= h - 1 => None,
                    Dir::Right if pos.1 >= w - 1 => None,
                    Dir::Up => Some((pos.0 - 1, pos.1)),
                    Dir::Down => Some((pos.0 + 1, pos.1)),
                    Dir::Left => Some((pos.0, pos.1 - 1)),
                    Dir::Right => Some((pos.0, pos.1 + 1)),
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
                if seen[sl][ix] {
                    result += 1;
                    break;
                }
            }
        }
        result
    }

    let grid: &[u8] = input.as_bytes();
    let w = grid.iter().position(|c| *c == b'\n').unwrap();
    let h = (grid.len() / w) - 1;

    let mut exits = FxHashSet::with_hasher(Default::default());
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if exits.contains(&(pos, dir.opposite())) {
            None
        } else {
            Some(traverse_lincache(grid, h, w, pos, dir, &mut exits))
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

/// Version 4.
///
/// We can squeeze another couple percent out of this approach by skipping bounds checks. This
/// is unsafe, and has no real practical utility other than go fast.
///
/// Mostly done bc I was curious about how big a difference bounds checking actually does compared
/// to unchecked access. I think checked access is plenty fast for me, personally.
///
/// I also made the functions on the `Dir` enum use unsafe bit twiddling as part of this. Mostly
/// because it's fun, since there's literally only 4 cases that can happen.
pub fn part2_unsafe(input: &str) -> i64 {
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

                let tar = match dir {
                    Dir::Up if pos.0 == 0 => None,
                    Dir::Left if pos.1 == 0 => None,
                    Dir::Down if pos.0 >= h - 1 => None,
                    Dir::Right if pos.1 >= w - 1 => None,
                    Dir::Up => Some((pos.0 - 1, pos.1)),
                    Dir::Down => Some((pos.0 + 1, pos.1)),
                    Dir::Left => Some((pos.0, pos.1 - 1)),
                    Dir::Right => Some((pos.0, pos.1 + 1)),
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

/// Version 5.
///
/// This one is actually slower; it just uses less memory. I was hoping for some cache-related
/// wins that didn't materialize.
///
/// However, I've realized we can probably speed up the summing step at the end!
pub fn part2_bitmap(input: &str) -> i64 {
    fn traverse_bitmap(
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
            vec![0u32; (h * w) / 32 + 1],
            vec![0u32; (h * w) / 32 + 1],
            vec![0u32; (h * w) / 32 + 1],
            vec![0u32; (h * w) / 32 + 1],
        ];
        while let Some((mut pos, mut dir)) = beams.pop_front() {
            let mut sl = unsafe { seen.get_unchecked_mut(dir as usize) };
            loop {
                let ix = pos.0 * h + pos.1;
                let mask = 1u32 << (ix % 32);
                let s = &mut sl[ix / 32];
                if (*s & mask) != 0 {
                    break;
                }
                *s |= mask;
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

                let tar = match dir {
                    Dir::Up if pos.0 == 0 => None,
                    Dir::Left if pos.1 == 0 => None,
                    Dir::Down if pos.0 >= h - 1 => None,
                    Dir::Right if pos.1 >= w - 1 => None,
                    Dir::Up => Some((pos.0 - 1, pos.1)),
                    Dir::Down => Some((pos.0 + 1, pos.1)),
                    Dir::Left => Some((pos.0, pos.1 - 1)),
                    Dir::Right => Some((pos.0, pos.1 + 1)),
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
            for dir in 0..4 {
                let sl = unsafe { seen.get_unchecked_mut(dir as usize) };
                let mask = 1u32 << (ix % 32);
                let s = &mut sl[ix / 32];
                if (*s & mask) != 0 {
                    result += 1;
                    break;
                }
            }
        }
        result as i64
    }

    let grid: &[u8] = input.as_bytes();
    let w = grid.iter().position(|c| *c == b'\n').unwrap();
    let h = (grid.len() / w) - 1;

    let mut exits = FxHashSet::with_hasher(Default::default());
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if exits.contains(&(pos, dir.opposite())) {
            None
        } else {
            Some(traverse_bitmap(grid, h, w, pos, dir, &mut exits))
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

/// Version 6.
///
/// It worked! Now that we can sum with a bitwise OR and count_digits it's now about twice as fast
/// on my computer (3.4ms/iter). The summing takes 1/64 as many iterations because we can check 64
/// entries in the bitmap at a time.
///
/// Also just OR'ing together each set of 64 positions in one instruction ends up faster than
/// looping over them individually and branching/incrementing for each one.
pub fn part2_popcnt(input: &str) -> i64 {
    fn traverse_popcnt(
        grid: &[u8],
        h: usize,
        w: usize,
        pos: Pos,
        dir: Dir,
        exits: &mut FxHashSet<(Pos, Dir)>,
    ) -> i64 {
        type B = u64;
        const MAP_SIZE: u64 = 64;
        let mut beams = VecDeque::<(Pos, Dir)>::new();
        beams.push_back((pos, dir));

        let mut seen: [Vec<B>; 4] = [
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
        ];
        while let Some((mut pos, mut dir)) = beams.pop_front() {
            let mut sl = &mut seen[dir as usize];
            loop {
                let ix = pos.0 * h + pos.1;
                let mask: B = 1 << (ix % MAP_SIZE as usize);
                let s = &mut sl[ix / MAP_SIZE as usize];
                if (*s & mask) != 0 {
                    break;
                }
                *s |= mask;
                static TILT_LUT: [Dir; 4] = [Dir::Right, Dir::Up, Dir::Left, Dir::Down];
                match grid[ix + pos.0] {
                    b'|' if dir.horizontal() => {
                        beams.push_back((pos, Dir::Up));
                        dir = Dir::Down;
                        sl = &mut seen[dir as usize];
                    }
                    b'-' if dir.vertical() => {
                        beams.push_back((pos, Dir::Left));
                        dir = Dir::Right;
                        sl = &mut seen[dir as usize];
                    }
                    b'/' => {
                        dir = TILT_LUT[dir as usize];
                        sl = &mut seen[dir as usize];
                    }
                    b'\\' => {
                        dir = TILT_LUT[(dir as usize + 2) & 0b11];
                        sl = &mut seen[dir as usize];
                    }
                    _ => {}
                }

                let tar = match dir {
                    Dir::Up if pos.0 == 0 => None,
                    Dir::Left if pos.1 == 0 => None,
                    Dir::Down if pos.0 >= h - 1 => None,
                    Dir::Right if pos.1 >= w - 1 => None,
                    Dir::Up => Some((pos.0 - 1, pos.1)),
                    Dir::Down => Some((pos.0 + 1, pos.1)),
                    Dir::Left => Some((pos.0, pos.1 - 1)),
                    Dir::Right => Some((pos.0, pos.1 + 1)),
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
        for ix in 0..((h * w) / MAP_SIZE as usize + 1) {
            let v = seen[0][ix] | seen[1][ix] | seen[2][ix] | seen[3][ix];
            result += v.count_ones();
        }
        result as i64
    }

    let grid: &[u8] = input.as_bytes();
    let w = grid.iter().position(|c| *c == b'\n').unwrap();
    let h = (grid.len() / w) - 1;

    let mut exits = FxHashSet::with_hasher(Default::default());
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if exits.contains(&(pos, dir.opposite())) {
            None
        } else {
            Some(traverse_popcnt(grid, h, w, pos, dir, &mut exits))
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

pub fn part2_popcnt_parallel(input: &str) -> i64 {
    fn traverse_popcnt(grid: &[u8], h: usize, w: usize, pos: Pos, dir: Dir) -> i64 {
        type B = u64;
        const MAP_SIZE: u64 = 64;
        let mut beams = VecDeque::<(Pos, Dir)>::new();
        beams.push_back((pos, dir));

        let mut seen: [Vec<B>; 4] = [
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
            vec![0; (h * w) / MAP_SIZE as usize + 1],
        ];
        while let Some((mut pos, mut dir)) = beams.pop_front() {
            let mut sl = &mut seen[dir as usize];
            loop {
                let ix = pos.0 * h + pos.1;
                let mask: B = 1 << (ix % MAP_SIZE as usize);
                let s = &mut sl[ix / MAP_SIZE as usize];
                if (*s & mask) != 0 {
                    break;
                }
                *s |= mask;
                static TILT_LUT: [Dir; 4] = [Dir::Right, Dir::Up, Dir::Left, Dir::Down];
                match grid[ix + pos.0] {
                    b'|' if dir.horizontal() => {
                        beams.push_back((pos, Dir::Up));
                        dir = Dir::Down;
                        sl = &mut seen[dir as usize];
                    }
                    b'-' if dir.vertical() => {
                        beams.push_back((pos, Dir::Left));
                        dir = Dir::Right;
                        sl = &mut seen[dir as usize];
                    }
                    b'/' => {
                        dir = TILT_LUT[dir as usize];
                        sl = &mut seen[dir as usize];
                    }
                    b'\\' => {
                        dir = TILT_LUT[(dir as usize + 2) & 0b11];
                        sl = &mut seen[dir as usize];
                    }
                    _ => {}
                }

                let tar = match dir {
                    Dir::Up if pos.0 == 0 => None,
                    Dir::Left if pos.1 == 0 => None,
                    Dir::Down if pos.0 >= h - 1 => None,
                    Dir::Right if pos.1 >= w - 1 => None,
                    Dir::Up => Some((pos.0 - 1, pos.1)),
                    Dir::Down => Some((pos.0 + 1, pos.1)),
                    Dir::Left => Some((pos.0, pos.1 - 1)),
                    Dir::Right => Some((pos.0, pos.1 + 1)),
                };
                if let Some(new_pos) = tar {
                    pos = new_pos;
                } else {
                    break;
                }
            }
        }
        let mut result = 0;
        for ix in 0..((h * w) / MAP_SIZE as usize + 1) {
            let v = seen[0][ix] | seen[1][ix] | seen[2][ix] | seen[3][ix];
            result += v.count_ones();
        }
        result as i64
    }

    let grid: &[u8] = input.as_bytes();
    let w = grid.iter().position(|c| *c == b'\n').unwrap();
    let h = (grid.len() / w) - 1;

    let traverse_from = |pos: Pos, dir: Dir| traverse_popcnt(grid, h, w, pos, dir);

    let entry_points = (0..h)
        .map(|row| ((row, 0), Dir::Right))
        .chain((0..h).map(|row| ((row, h - 1), Dir::Left)))
        .chain((0..w).map(|col| ((0, col), Dir::Down)))
        .chain((0..w).map(|col| ((w - 1, col), Dir::Up)))
        .collect::<Vec<_>>();
    entry_points
        .into_par_iter()
        .map(|(pos, dir)| traverse_from(pos, dir))
        .max()
        .unwrap()
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![
        ("part1", part1),
        ("part2", part2),
        ("part2 (opt)", part2_opt),
        ("part2 (linear cache)", part2_lincache),
        ("part2 (unsafe)", part2_unsafe),
        ("part2 (bitmaps)", part2_bitmap),
        ("part2 (popcnt)", part2_popcnt),
        ("part2 (popcnt, parallel)", part2_popcnt_parallel),
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
    assert_eq!(part2_opt(&input), 7759);
    assert_eq!(part2_lincache(&input), 7759);
    assert_eq!(part2_unsafe(&input), 7759);
    assert_eq!(part2_bitmap(&input), 7759);
    assert_eq!(part2_popcnt(&input), 7759);
    assert_eq!(part2_popcnt_parallel(&input), 7759);
}
