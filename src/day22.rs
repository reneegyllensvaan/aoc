use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use crate::utils::range_intersect;

static INPUT_FILE: &str = "input/day22";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}
impl Point {
    fn parse(v: &str) -> Self {
        let mut tok = v.split(',');
        Self {
            x: tok.next().unwrap().parse().unwrap(),
            y: tok.next().unwrap().parse().unwrap(),
            z: tok.next().unwrap().parse().unwrap(),
        }
    }
}
#[derive(Debug)]
struct Brick {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}
impl Brick {
    fn supports(&self, other: &Self) -> bool {
        let connect_in_hz_plane = range_intersect(&self.x, &other.x).is_some()
            && range_intersect(&self.y, &other.y).is_some();
        let result = connect_in_hz_plane && (self.z.end() + 1 == *other.z.start());
        result
    }
}

pub fn part1(input: &str) -> i64 {
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            let start = Point::parse(start);
            let end = Point::parse(end);
            Brick {
                x: start.x.min(end.x)..=start.x.max(end.x),
                y: start.y.min(end.y)..=start.y.max(end.y),
                z: start.z.min(end.z)..=start.z.max(end.z),
            }
        })
        .collect();
    // TODO: explore using this kinda property to skip checking most blocks when dropping
    bricks.sort_unstable_by_key(|v| *v.z.end());

    // first, i guess we let all bricks drop to their resting state
    let mut changed = true;
    while changed {
        let mut drop = vec![];
        for (ix, a) in bricks.iter().enumerate() {
            if *a.z.start() != 0 && !bricks.iter().any(|b| b.supports(a)) {
                drop.push(ix);
            }
        }
        changed = drop.len() > 0;
        for drop_ix in drop {
            let d = &mut bricks[drop_ix];
            d.z = (d.z.start() - 1)..=(d.z.end() - 1);
        }
    }

    let mut supported_by = HashMap::<usize, Vec<usize>>::new();
    for (ix_a, a) in bricks.iter().enumerate() {
        for (ix_b, b) in bricks.iter().enumerate() {
            if a.supports(b) {
                supported_by.entry(ix_b).or_default().push(ix_a);
            }
        }
    }

    let mut sole_supporters = (0..bricks.len()).collect::<HashSet<_>>();
    for supporters in supported_by.values() {
        if let [s] = supporters.as_slice() {
            sole_supporters.remove(s);
        }
    }

    sole_supporters.len() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut bricks: Vec<Brick> = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            let start = Point::parse(start);
            let end = Point::parse(end);
            Brick {
                x: start.x.min(end.x)..=start.x.max(end.x),
                y: start.y.min(end.y)..=start.y.max(end.y),
                z: start.z.min(end.z)..=start.z.max(end.z),
            }
        })
        .collect();

    // first, i guess we let all bricks drop to their resting state
    let mut changed = true;
    while changed {
        let mut drop = vec![];
        for (ix, a) in bricks.iter().enumerate() {
            if *a.z.start() != 0 && !bricks.iter().any(|b| b.supports(a)) {
                drop.push(ix);
            }
        }
        changed = drop.len() > 0;
        for drop_ix in drop {
            let d = &mut bricks[drop_ix];
            d.z = (d.z.start() - 1)..=(d.z.end() - 1);
        }
    }

    let mut supported_by = HashMap::<usize, Vec<usize>>::new();
    let mut supporters = HashMap::<usize, Vec<usize>>::new();
    for (ix_a, a) in bricks.iter().enumerate() {
        for (ix_b, b) in bricks.iter().enumerate() {
            if a.supports(b) {
                supporters.entry(ix_a).or_default().push(ix_b);
                supported_by.entry(ix_b).or_default().push(ix_a);
            }
        }
    }

    let mut result = 0;
    for (ix, _) in bricks.iter().enumerate() {
        let mut falling = HashSet::<usize>::new();
        let mut q = VecDeque::new();
        q.push_back(ix);
        falling.insert(ix);
        while let Some(n) = q.pop_front() {
            if let Some(supporters) = supported_by.get(&n) {
                if supporters.iter().all(|s| falling.contains(s)) {
                    falling.insert(n);
                }
            };
            if let Some(supportees) = supporters.get(&n) {
                for supportee in supportees {
                    q.push_back(*supportee);
                }
            };
        }
        let n = falling.len() - 1;
        result += n;
    }

    result as i64
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 1;

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
    assert_eq!(part1(EXAMPLE_INPUT), 5);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 7);
}

// #[test]
// fn test_part1_facit() {
//     let input = std::fs::read_to_string(INPUT_FILE).unwrap();
//     assert_eq!(part1(&input), 386);
// }

// #[test]
// fn test_part2_facit() {
//     let input = std::fs::read_to_string(INPUT_FILE).unwrap();
//     assert_eq!(part2(&input), 0);
// }

#[test]
fn test_supports_iself() {
    // brick should never support itself
    let a = Brick {
        x: 1..=1,
        y: 0..=2,
        z: 1..=1,
    };
    let b = Brick {
        x: 1..=1,
        y: 0..=2,
        z: 1..=1,
    };
    assert_eq!(a.supports(&b), false);
}

#[test]
fn test_supports_truthy() {
    // a that does support b
    let a = Brick {
        x: 1..=2,
        y: 0..=2,
        z: 1..=1,
    };
    let b = Brick {
        x: 1..=1,
        y: 0..=2,
        z: 2..=3,
    };
    assert_eq!(a.supports(&b), true);
}
