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

/// This one was basically translated line-by-line from this post:
/// https://forums.swift.org/t/advent-of-code-2023/68749/61
///
/// I just wanted to get it in a format i can understand so i can analyze it. It's a very cool
/// approach!
pub fn part2_dfa(contents: &str) -> i64 {
    enum NodeType {
        StartOfGroup,
        MidGroup,
        EndOfGroup,
        Finished,
    }
    impl NodeType {
        fn operational_move(&self) -> Option<usize> {
            match self {
                Self::StartOfGroup | Self::Finished => Some(0),
                Self::MidGroup => None,
                Self::EndOfGroup => Some(1),
            }
        }
        fn damaged_move(&self) -> Option<usize> {
            match self {
                Self::StartOfGroup | Self::MidGroup => Some(1),
                Self::EndOfGroup | Self::Finished => None,
            }
        }
    }

    fn make_node_types(groups: &[usize]) -> Vec<NodeType> {
        let mut result: Vec<NodeType> = vec![];
        for group in groups {
            result.push(NodeType::StartOfGroup);
            for _ in 0..(group - 1) {
                result.push(NodeType::MidGroup);
            }
            result.push(NodeType::EndOfGroup);
        }
        result.push(NodeType::Finished);
        return result;
    }

    fn next(c: char, nodes: &[NodeType], counts: &mut [usize]) {
        let operation_move = c != '#';
        let damage_move = c != '.';
        for (i, node) in nodes.iter().enumerate().rev() {
            let n = counts[i];
            if n <= 0 {
                continue;
            }
            counts[i] = 0;
            if damage_move {
                if let Some(m) = node.damaged_move() {
                    counts[i + m] += n;
                }
            }
            if operation_move {
                if let Some(m) = nodes[i].operational_move() {
                    counts[i + m] += n;
                }
            }
        }
    }

    let mut total = 0;
    for line in contents.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_vec();

        let mut long_groups: Vec<usize> = vec![];
        for _ in 0..5 {
            long_groups.append(&mut groups.clone());
        }
        let nodes = make_node_types(&long_groups);
        let mut counts = vec![0; nodes.len()];
        counts[0] = 1;
        for i in 0..5 {
            for c in springs.chars() {
                next(c, &nodes, &mut counts);
            }
            if i != 4 {
                next('?', &nodes, &mut counts);
            }
        }
        total += counts[nodes.len() - 1] + counts[nodes.len() - 2]
    }
    return total as i64;
}
pub fn part1_dfa(contents: &str) -> i64 {
    enum NodeType {
        StartOfGroup,
        MidGroup,
        EndOfGroup,
        Finished,
    }
    impl NodeType {
        fn operational_move(&self) -> Option<usize> {
            match self {
                Self::StartOfGroup | Self::Finished => Some(0),
                Self::MidGroup => None,
                Self::EndOfGroup => Some(1),
            }
        }
        fn damaged_move(&self) -> Option<usize> {
            match self {
                Self::StartOfGroup | Self::MidGroup => Some(1),
                Self::EndOfGroup | Self::Finished => None,
            }
        }
    }

    fn make_node_types(groups: &[usize]) -> Vec<NodeType> {
        let mut result: Vec<NodeType> = vec![];
        for group in groups {
            result.push(NodeType::StartOfGroup);
            for _ in 0..(group - 1) {
                result.push(NodeType::MidGroup);
            }
            result.push(NodeType::EndOfGroup);
        }
        result.push(NodeType::Finished);
        return result;
    }

    fn next(c: char, nodes: &[NodeType], counts: &mut [usize]) {
        let operation_move = c != '#';
        let damage_move = c != '.';
        for (i, node) in nodes.iter().enumerate().rev() {
            let n = counts[i];
            if n <= 0 {
                continue;
            }
            counts[i] = 0;
            if damage_move {
                if let Some(m) = node.damaged_move() {
                    counts[i + m] += n;
                }
            }
            if operation_move {
                if let Some(m) = node.operational_move() {
                    counts[i + m] += n;
                }
            }
        }
    }

    let mut total = 0;
    for line in contents.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_vec();

        let nodes = make_node_types(&groups);
        let mut counts = vec![0; nodes.len()];
        counts[0] = 1;
        for c in springs.chars() {
            next(c, &nodes, &mut counts);
        }
        total += counts[nodes.len() - 1] + counts[nodes.len() - 2]
    }
    return total as i64;
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string("input/day12").unwrap();

    let iters = 100;

    let fns: [(&'static str, fn(&str) -> i64); 4] = [
        ("part1", part1),
        ("part1 (dfa)", part1_dfa),
        ("part2", part2),
        ("part2 (dfa)", part2_dfa),
    ];

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
    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    assert_eq!(part1(input), 21);
    assert_eq!(part1_dfa(input), 21);
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
    assert_eq!(part2_dfa(input), 525152);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    assert_eq!(part1(&input), 7506);
    assert_eq!(part1_dfa(&input), 7506);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    assert_eq!(part2(&input), 548241300348335);
    assert_eq!(part2_dfa(&input), 548241300348335);
}
