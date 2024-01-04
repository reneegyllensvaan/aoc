use std::collections::HashSet;

use itertools::Itertools;

static INPUT_FILE: &str = "input/day03";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    for line in input.as_bytes().split(|c| *c == b'\n') {
        let (l, r) = line.split_at(line.len() / 2);
        let mut line_result = 0;
        for a in l {
            for b in r {
                if a == b {
                    let prio = match a {
                        b'a'..=b'z' => a - b'a' + 1,
                        b'A'..=b'Z' => a - b'A' + 27,
                        _ => panic!("invalid char: {a:?}"),
                    };
                    line_result = line_result.max(prio);
                    break;
                }
            }
        }
        result += line_result as i64;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    for lines in &(input.as_bytes().split(|c| *c == b'\n')).chunks(3) {
        let bags: Vec<HashSet<u8>> = lines
            .into_iter()
            .map(|l| l.into_iter().copied().collect())
            .collect();
        let mut group_result = 0;
        for c in bags[0].iter() {
            if bags.iter().all(|b| b.contains(c)) {
                let prio = match c {
                    b'a'..=b'z' => c - b'a' + 1,
                    b'A'..=b'Z' => c - b'A' + 27,
                    _ => panic!("invalid char: {c:?}"),
                };
                group_result = group_result.max(prio);
                break;
            }
        }
        result += group_result as i64;
    }
    result
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
    assert_eq!(part1(EXAMPLE_INPUT), 157);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 70);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 7821);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 2752);
}
