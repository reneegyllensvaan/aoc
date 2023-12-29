pub type Grid<T> = Vec<Vec<T>>;
pub type SGrid<T> = [Vec<T>];

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
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
        match self {
            Self::Left | Self::Right => true,
            _ => false,
        }
    }
    pub fn vertical(self) -> bool {
        !self.horizontal()
    }
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    pub fn turn_right(self) -> Self {
        self.turn_left().opposite()
    }
}

pub type Pos = (usize, usize);
pub trait PosUtils {
    fn go(&self, dir: Dir) -> Option<Pos>;
    fn go_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos>;
    fn go_wrapping_in<T: Clone>(&self, dir: Dir, grid: &SGrid<T>) -> Option<Pos>;
    fn neighbors_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos>;
    fn neighbors_wrapping_in<T: Clone>(&self, grid: &SGrid<T>) -> Vec<Pos>;
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
