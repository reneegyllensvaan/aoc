use std::{mem::transmute, ops::RangeInclusive};

pub type Grid<T> = Vec<Vec<T>>;
pub type SGrid<T> = [Vec<T>];

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
#[repr(u8)]
pub enum Dir {
    Up = 0b00,
    Right = 0b01,
    Down = 0b10,
    Left = 0b11,
}
impl Dir {
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            d => panic!("unknown direction: {d}"),
        }
    }
    pub fn horizontal(self) -> bool {
        (self as u8) & 0b01 == 1
    }
    pub fn vertical(self) -> bool {
        (self as u8) & 0b01 == 0
    }
    pub fn opposite(self) -> Self {
        unsafe { transmute((self as u8 + 2) & 0b11) }
    }
    pub fn turn_right(self) -> Self {
        unsafe { transmute((self as u8 + 1) & 0b11) }
    }
    pub fn turn_left(self) -> Self {
        unsafe { transmute((self as u8 + 3) & 0b11) }
    }
}
#[test]
fn dir_turn_left() {
    assert_eq!(Dir::Up.turn_left(), Dir::Left);
    assert_eq!(Dir::Down.turn_left(), Dir::Right);
    assert_eq!(Dir::Left.turn_left(), Dir::Down);
    assert_eq!(Dir::Right.turn_left(), Dir::Up);
}
#[test]
fn dir_turn_right() {
    assert_eq!(Dir::Up.turn_right(), Dir::Right);
    assert_eq!(Dir::Down.turn_right(), Dir::Left);
    assert_eq!(Dir::Left.turn_right(), Dir::Up);
    assert_eq!(Dir::Right.turn_right(), Dir::Down);
}
#[test]
fn dir_turn_horizontal() {
    assert_eq!(Dir::Up.horizontal(), false);
    assert_eq!(Dir::Down.horizontal(), false);
    assert_eq!(Dir::Left.horizontal(), true);
    assert_eq!(Dir::Right.horizontal(), true);
}
#[test]
fn dir_turn_vertical() {
    assert_eq!(Dir::Up.vertical(), true);
    assert_eq!(Dir::Down.vertical(), true);
    assert_eq!(Dir::Left.vertical(), false);
    assert_eq!(Dir::Right.vertical(), false);
}

pub type Pos = (usize, usize);
pub trait PosUtils {
    fn go(&self, dir: Dir) -> Option<Pos>;
    fn go_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos>;
    fn go_wrapping_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos>;
    fn neighbors_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos>;
    fn neighbors_wrapping_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos>;
    fn dir_of(&self, other: &Self) -> Dir;
}
impl PosUtils for Pos {
    fn go(&self, dir: Dir) -> Option<Pos> {
        let (r, c) = *self;
        match dir {
            Dir::Up if r == 0 => None,
            Dir::Left if c == 0 => None,
            Dir::Up => Some((r - 1, c)),
            Dir::Down => Some((r + 1, c)),
            Dir::Left => Some((r, c - 1)),
            Dir::Right => Some((r, c + 1)),
        }
    }
    fn go_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos> {
        if grid.len() == 0 {
            return None;
        }
        let (r, c) = *self;
        let h = grid.len();
        let w = grid[0].len();
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
    }
    fn go_wrapping_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos> {
        if grid.len() == 0 {
            return None;
        }
        let (r, c) = *self;
        let h = grid.len();
        let w = grid[0].len();
        match dir {
            Dir::Up if r == 0 => Some((h - 1, c)),
            Dir::Left if c == 0 => Some((r, w - 1)),
            Dir::Down if r >= h - 1 => Some((0, c)),
            Dir::Right if c >= w - 1 => Some((r, 0)),
            Dir::Up => Some((r - 1, c)),
            Dir::Down => Some((r + 1, c)),
            Dir::Left => Some((r, c - 1)),
            Dir::Right => Some((r, c + 1)),
        }
    }
    fn neighbors_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .flat_map(|dir| self.go_in(dir, grid))
            .collect()
    }
    fn neighbors_wrapping_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .flat_map(|dir| self.go_wrapping_in(dir, grid))
            .collect()
    }
    fn dir_of(&self, other: &Self) -> Dir {
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            if self.go(dir).is_some_and(|v| v == *other) {
                return dir;
            }
        }
        panic!("other is not a neighbor");
    }
}

pub trait Vec2dUtils<T: Clone> {
    fn transpose(&self) -> Vec<Vec<T>>;
    fn at(&self, pos: Pos) -> T;
}

impl<T: Clone> Vec2dUtils<T> for [Vec<T>] {
    fn at(&self, pos: Pos) -> T {
        self[pos.0][pos.1].clone()
    }
    /// Takes a 2d vector and transposes its axes such that columns become rows and rows become
    /// columns, moves each index (x, y) to (y, x)
    fn transpose(&self) -> Vec<Vec<T>> {
        let Some(first) = self.first() else {
            return Vec::new();
        };
        (0..first.len())
            .map(|i| {
                self.iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

pub fn range_intersect(
    a: &RangeInclusive<i64>,
    b: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    let start = *a.start().max(b.start());
    let end = *a.end().min(b.end());
    if start <= end {
        Some(start..=end)
    } else {
        None
    }
}
#[test]
fn test_range_intersect() {
    assert_eq!(range_intersect(&(1..=2), &(1..=1)), Some(1..=1));
    assert_eq!(range_intersect(&(10..=50), &(5..=15)), Some(10..=15));
    assert_eq!(range_intersect(&(5..=50), &(10..=15)), Some(10..=15));
    assert_eq!(range_intersect(&(12..=20), &(10..=15)), Some(12..=15));
    assert_eq!(range_intersect(&(4..=5), &(10..=15)), None);
}

pub fn separate_thousands(n: &str) -> String {
    n.chars()
        .collect::<Vec<_>>()
        .rchunks(3)
        .rev()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("_")
}
#[test]
fn test_separate_thousands() {
    assert_eq!(separate_thousands("1000000"), "1_000_000");
    assert_eq!(separate_thousands("100000"), "100_000");
    assert_eq!(separate_thousands("1000"), "1_000");
    assert_eq!(separate_thousands("100"), "100");
}
