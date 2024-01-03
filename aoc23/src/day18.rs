use crate::utils::Dir;

static INPUT_FILE: &str = "input/day18";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

pub fn part1(input: &str) -> i64 {
    let instructions: Vec<(Dir, i64)> = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            (
                Dir::from_char(l.next().unwrap().chars().next().unwrap()),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut points: Vec<(i64, i64)> = vec![(0, 0)];
    for (dir, len) in instructions.iter() {
        let prev = points.last().unwrap().clone();
        let new_pos = match dir {
            Dir::Up => (prev.0 - len, prev.1),
            Dir::Down => (prev.0 + len, prev.1),
            Dir::Left => (prev.0, prev.1 + len),
            Dir::Right => (prev.0, prev.1 - len),
        };
        points.push(new_pos);
    }

    // trapezoid formula baybey!!
    let area: i64 = points
        .iter()
        .copied()
        .zip(points.iter().copied().cycle().skip(1))
        .map(|((y1, x1), (y2, x2))| (y2 + y1) * (x2 - x1))
        .sum::<i64>();

    let perimeter: i64 = instructions.iter().map(|v| v.1).sum();

    1 + (area + perimeter) / 2
}

pub fn part2(input: &str) -> i64 {
    let instructions: Vec<(Dir, i64)> = input
        .lines()
        .map(|l| {
            let s = &l[l.len() - 7..l.len() - 1];
            let dir = match s.chars().rev().next().unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                d => panic!("unexpected direction code: {d}"),
            };
            let len = i64::from_str_radix(&s[0..s.len() - 1], 16).unwrap();
            (dir, len)
        })
        .collect();

    let mut points: Vec<(i64, i64)> = vec![(0, 0)];
    for (dir, len) in instructions.iter() {
        let prev = points.last().unwrap().clone();
        let new_pos = match dir {
            Dir::Up => (prev.0 - len, prev.1),
            Dir::Down => (prev.0 + len, prev.1),
            Dir::Left => (prev.0, prev.1 + len),
            Dir::Right => (prev.0, prev.1 - len),
        };
        points.push(new_pos);
    }

    // trapezoid formula baybey!!
    let area: i64 = points
        .iter()
        .copied()
        .zip(points.iter().copied().cycle().skip(1))
        .map(|((y1, x1), (y2, x2))| (y2 + y1) * (x2 - x1))
        .sum::<i64>();

    let perimeter: i64 = instructions.iter().map(|v| v.1).sum();

    1 + (area + perimeter) / 2
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1), ("part2", part2)];

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
    assert_eq!(part1(EXAMPLE_INPUT), 62);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 952408144115);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 33491);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 87716969654406);
}
