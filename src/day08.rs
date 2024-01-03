use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    let mut paths = HashMap::<&str, (&str, &str)>::new();
    let mut lines = input.lines();
    let mut instr = lines.next().unwrap().chars().cycle();
    lines.next();

    for line in lines {
        paths.insert(&line[0..=2], (&line[7..=9], &line[12..=14]));
    }

    let mut result = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        let (l, r) = paths.get(pos).unwrap();
        pos = match instr.next().unwrap() {
            'L' => l,
            'R' => r,
            v => panic!("invalid path direction: {v}"),
        };
        result += 1;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut paths = HashMap::<&str, (&str, &str)>::new();
    let mut lines = input.lines();
    let instr = lines.next().unwrap().chars().cycle();
    lines.next();

    for line in lines {
        paths.insert(&line[0..=2], (&line[7..=9], &line[12..=14]));
    }

    paths
        .keys()
        .filter(|v| v.ends_with('A'))
        .map(|p| {
            let mut pos = *p;
            let mut instr = instr.clone();
            let mut its = 0;
            while !pos.ends_with('Z') {
                let (l, r) = paths.get(pos).unwrap();
                pos = match instr.next().unwrap() {
                    'L' => l,
                    'R' => r,
                    v => panic!("invalid path direction: {v}"),
                };
                its += 1;
            }
            its
        })
        .fold(1, num::integer::lcm)
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string("input/day08").unwrap();

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 2] = [("part1", part1), ("part2", part2)];

    for (name, f) in fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
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
}

#[test]
fn test_part1_example() {
    let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
    assert_eq!(part1(input), 2);
}

#[test]
fn test_part1_example_2() {
    let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2_example() {
    let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    assert_eq!(part2(input), 6);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day08").unwrap();
    assert_eq!(part1(&input), 21797);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day08").unwrap();
    assert_eq!(part2(&input), 23977527174353);
}
