use std::collections::HashMap;

use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        let (row, seqs) = line.split_once(' ').unwrap();
        let seqs: Vec<i64> = seqs.split(',').map(|v| v.parse().unwrap()).collect();

        result += solve(&row, &seqs, &mut HashMap::new());
    }
    result
}

fn solve(tail: &str, seqs: &[i64], cache: &mut HashMap<(*const u8, *const i64), i64>) -> i64 {
    if tail.is_empty() {
        return if seqs.is_empty() { 1 } else { 0 };
    }
    if seqs.is_empty() {
        return if tail.chars().any(|c| c == '#') { 0 } else { 1 };
    }

    // lmao what am i doing
    if let Some(result) = cache.get(&(tail.as_ptr(), seqs.as_ptr())) {
        return *result;
    }

    let mut result = 0;
    if tail.starts_with(&['#', '?']) {
        let b_len = *seqs.first().unwrap() as usize;
        if b_len <= tail.len()
            && !tail.chars().take(b_len).any(|c| c == '.')
            && (b_len == tail.len() || !tail[b_len..].starts_with('#'))
        {
            let rem = &tail[(b_len + 1).min(tail.len())..];
            let s = &seqs[1..];
            result += solve(rem, s, cache);
        }
    }
    if tail.starts_with(&['.', '?']) {
        result += solve(&tail[1..], seqs, cache);
    }

    cache.insert((tail.as_ptr(), seqs.as_ptr()), result);

    result
}

pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        let (row, seqs) = line.split_once(' ').unwrap();
        let seqs: Vec<i64> = seqs.split(',').map(|v| v.parse().unwrap()).collect();
        let row = [row, row, row, row, row].into_iter().join("?");
        let seqs: Vec<i64> = [&seqs, &seqs, &seqs, &seqs, &seqs]
            .into_iter()
            .flatten()
            .copied()
            .collect();

        result += solve(&row, &seqs, &mut HashMap::new());
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("input/day12").unwrap();

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
    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    assert_eq!(part1(input), 21);
}

#[test]
fn test_part1_example_line_1() {
    assert_eq!(part1("???.### 1,1,3"), 1);
}
#[test]
fn test_part1_example_line_2() {
    assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
}
#[test]
fn test_part1_example_line_3() {
    assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
}
#[test]
fn test_part1_example_line_4() {
    assert_eq!(part1("????.#...#... 4,1,1"), 1);
}
#[test]
fn test_part1_example_line_5() {
    assert_eq!(part1("????.######..#####. 1,6,5"), 4);
}
#[test]
fn test_part1_example_line_6() {
    assert_eq!(part1("?###???????? 3,2,1"), 10);
}

#[test]
fn test_part2_example() {
    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    assert_eq!(part2(input), 525152);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    assert_eq!(part1(&input), 7506);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    assert_eq!(part2(&input), 548241300348335);
}
