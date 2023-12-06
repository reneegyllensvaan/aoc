use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::Range,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartNumber {
    num: i32,
    row: i32,
    span: Range<i32>,
}

/// Version 1.
///
/// This sort of has two versions by flipping which parser the `parse` function calls.
///
/// Altogether a pretty naive solution, but runs plenty well.
pub fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse(input);

    for part in parts.iter() {
        for symbol in symbols.iter() {
            let within_row = part.row >= symbol.1 - 1 && part.row <= symbol.1 + 1;
            let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
            if within_row && within_span {
                result += part.num;
                break;
            }
        }
    }
    result
}

/// Version 2.
///
/// The optimization here is just that we pull the symbols into a B-tree, so that we can use that
/// to only handle adjacent lines.
///
/// A flame graph told me that not too much time is spent in parsing, so I didn't optimize that
/// section very much.
///
/// Another approach might be to use a linear collection, or even a hash table (since we know we
/// exactly have three rows to access). That might be more than a quick fix.
pub fn part1_btree(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse(input);
    let symbols: BTreeSet<(i32, i32)> = symbols
        .into_iter()
        .map(|(_, row, col)| (row, col))
        .collect();

    for part in parts.iter() {
        for symbol in symbols.range((part.row - 1, i32::MIN)..=(part.row + 1, i32::MAX)) {
            let within_span = symbol.1 >= part.span.start - 1 && symbol.1 <= part.span.end;
            if within_span {
                result += part.num;
                break;
            }
        }
    }
    result
}

/// Version 3.
///
/// Turns out, using a hash table was actually a little quicker!
///
/// I think B-tree iterators either don't pay for themselves at this size (they were big in the
/// flame graph), or insertion takes too long into the B-tree map.
pub fn part1_hash(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse(input);
    let symbols: HashMap<i32, Vec<i32>> = symbols
        .into_iter()
        .map(|(_, row, col)| (row, col))
        .into_group_map();

    for part in parts.iter() {
        let target_rows = [
            symbols.get(&(part.row - 1)),
            symbols.get(&part.row),
            symbols.get(&(part.row + 1)),
        ]
        .into_iter()
        .flatten();

        for symbols in target_rows {
            for symbol in symbols {
                let within_span = *symbol >= part.span.start - 1 && *symbol <= part.span.end;
                if within_span {
                    result += part.num;
                    break;
                }
            }
        }
    }
    result
}

/// Version 1.
///
/// Same approach as part1, just have to flip the loops.
pub fn part2(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse(input);

    for symbol in symbols.iter().filter(|(c, _, _)| *c == '*') {
        let mut adj: Vec<PartNumber> = Vec::new();
        for part in parts.iter() {
            let within_row = part.row >= symbol.1 - 1 && part.row <= symbol.1 + 1;
            let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
            if within_row && within_span {
                adj.push(part.clone());
            }
        }
        if let [a, b] = &adj[..] {
            result += a.num * b.num;
        }
    }
    result
}

/// Version 2.
///
/// Same optimization as part1_btree. Use a B-tree, grouping the parts by row, then iterate over
/// only the range of affected rows.
pub fn part2_btree(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts_vec, symbols) = parse(input);
    let mut parts: BTreeMap<i32, Vec<PartNumber>> = BTreeMap::new();
    for part in parts_vec {
        parts.entry(part.row).or_default().push(part);
    }

    for symbol in symbols.iter().filter(|(c, _, _)| *c == '*') {
        let mut adj: Vec<PartNumber> = Vec::new();
        for (_, row_parts) in parts.range((symbol.1 - 1)..=(symbol.1 + 1)) {
            for part in row_parts {
                // let within_row = part.row >= symbol.1 - 1 && part.row <= symbol.1 + 1;
                let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
                if within_span {
                    adj.push(part.clone());
                }
            }
        }
        if let [a, b] = &adj[..] {
            result += a.num * b.num;
        }
    }
    result
}

/// Version 3.
///
/// Same optimization as part1_hash. Having a bounded set of rows to check means we can check
/// faster with a hash map.
pub fn part2_hash(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse(input);
    let parts = parts.into_iter().map(|v| (v.row, v)).into_group_map();

    for symbol in symbols.iter().filter(|(c, _, _)| *c == '*') {
        let mut adj: Vec<PartNumber> = Vec::new();

        let target_rows = [
            parts.get(&(symbol.1 - 1)),
            parts.get(&symbol.1),
            parts.get(&(symbol.1 + 1)),
        ]
        .into_iter()
        .flatten();

        for row_parts in target_rows {
            for part in row_parts {
                let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
                if within_span {
                    adj.push(part.clone());
                }
            }
        }
        if let [a, b] = &adj[..] {
            result += a.num * b.num;
        }
    }
    result
}

/// Baseline benchmark of parsing with a regex
pub fn part2_regex(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse_regex(input);

    for symbol in symbols.iter().filter(|(c, _, _)| *c == '*') {
        let mut adj: Vec<PartNumber> = Vec::new();
        for part in parts.iter() {
            let within_row = part.row >= symbol.1 - 1 && part.row <= symbol.1 + 1;
            let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
            if within_row && within_span {
                adj.push(part.clone());
            }
        }
        if let [a, b] = &adj[..] {
            result += a.num * b.num;
        }
    }
    result
}

/// Baseline benchmark of parsing with the non-regex parser
pub fn part2_no_regex(input: &str) -> i32 {
    let mut result: i32 = 0;

    let (parts, symbols) = parse_no_regex(input);

    for symbol in symbols.iter().filter(|(c, _, _)| *c == '*') {
        let mut adj: Vec<PartNumber> = Vec::new();
        for part in parts.iter() {
            let within_row = part.row >= symbol.1 - 1 && part.row <= symbol.1 + 1;
            let within_span = symbol.2 >= part.span.start - 1 && symbol.2 <= part.span.end;
            if within_row && within_span {
                adj.push(part.clone());
            }
        }
        if let [a, b] = &adj[..] {
            result += a.num * b.num;
        }
    }
    result
}

fn parse(input: &str) -> (Vec<PartNumber>, Vec<(char, i32, i32)>) {
    parse_no_regex(input)
}

fn parse_regex(input: &str) -> (Vec<PartNumber>, Vec<(char, i32, i32)>) {
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<(char, i32, i32)> = Vec::new();

    let expr = regex::Regex::new(r"[0-9]+|[^0-9.]").unwrap();
    for (row, line) in input.lines().enumerate() {
        for m in expr.find_iter(line) {
            if let Ok(num) = m.as_str().parse::<i32>() {
                parts.push(PartNumber {
                    num,
                    row: row as i32,
                    span: (m.start() as i32)..(m.end() as i32),
                });
            } else {
                symbols.push((
                    m.as_str().chars().next().unwrap(),
                    row as i32,
                    m.start() as i32,
                ));
            }
        }
    }
    (parts, symbols)
}

fn parse_no_regex(input: &str) -> (Vec<PartNumber>, Vec<(char, i32, i32)>) {
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<(char, i32, i32)> = Vec::new();

    let mut tail = &input[..];
    let mut row = 0;
    let mut col = 0;
    while tail.len() > 0 {
        if tail.starts_with('.') {
            col += 1;
            tail = &tail[1..];
        } else if tail.starts_with(|c: char| c.is_digit(10)) {
            let s = if let Some((s, _)) = tail.split_once(|c: char| !c.is_digit(10)) {
                s
            } else {
                tail
            };
            let num: i32 = s.parse().unwrap();
            parts.push(PartNumber {
                num,
                row: row as i32,
                span: col..(col + s.len() as i32),
            });
            col += s.len() as i32;
            tail = &tail[s.len()..];
        } else if tail.starts_with('\n') {
            row += 1;
            col = 0;
            tail = &tail[1..];
        } else {
            let c = tail.chars().next().unwrap();
            symbols.push((c, row as i32, col));
            col += c.len_utf8() as i32;
            tail = &tail[c.len_utf8()..];
        }
    }
    (parts, symbols)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day03").unwrap();

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i32); 8] = [
        ("part1", part1),
        ("part2", part2),
        ("part1 (btree)", part1_btree),
        ("part1 (hash)", part1_hash),
        ("part2 (regex)", part2_regex),
        ("part2 (no regex)", part2_no_regex),
        ("part2 (btree)", part2_btree),
        ("part2 (hash)", part2_hash),
    ];

    for (name, f) in fns {
        println!("{name}: {}", f(&input));
    }

    for (name, f) in fns {
        let begin = std::time::Instant::now();
        for _ in 0..iters {
            f(&input);
        }
        let end = std::time::Instant::now();
        println!(
            "{} {} in: {}us ({}us/iter)",
            iters,
            name,
            (end - begin).as_micros(),
            (end - begin).as_micros() / iters
        );
    }
}

#[test]
fn test_part1_example() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part1(input), 4361);
    assert_eq!(part1_btree(input), 4361);
    assert_eq!(part1_hash(input), 4361);
}

#[test]
fn test_part2_example() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(part2(input), 467835);
    assert_eq!(part2_hash(input), 467835);
    assert_eq!(part2_btree(input), 467835);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day03").unwrap();
    assert_eq!(part1(&input), 536576);
    assert_eq!(part1_btree(&input), 536576);
    assert_eq!(part1_hash(&input), 536576);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day03").unwrap();
    assert_eq!(part2(&input), 75741499);
    assert_eq!(part2_btree(&input), 75741499);
    assert_eq!(part2_hash(&input), 75741499);
}

#[test]
fn test_parse() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    let expected_parts = vec![
        PartNumber {
            num: 467,
            row: 0,
            span: 0..3,
        },
        PartNumber {
            num: 114,
            row: 0,
            span: 5..8,
        },
        PartNumber {
            num: 35,
            row: 2,
            span: 2..4,
        },
        PartNumber {
            num: 633,
            row: 2,
            span: 6..9,
        },
        PartNumber {
            num: 617,
            row: 4,
            span: 0..3,
        },
        PartNumber {
            num: 58,
            row: 5,
            span: 7..9,
        },
        PartNumber {
            num: 592,
            row: 6,
            span: 2..5,
        },
        PartNumber {
            num: 755,
            row: 7,
            span: 6..9,
        },
        PartNumber {
            num: 664,
            row: 9,
            span: 1..4,
        },
        PartNumber {
            num: 598,
            row: 9,
            span: 5..8,
        },
    ];
    let expected_symbols = vec![
        ('*', 1, 3),
        ('#', 3, 6),
        ('*', 4, 3),
        ('+', 5, 5),
        ('$', 8, 3),
        ('*', 8, 5),
    ];
    let (parsed_parts, parsed_symbols) = parse_regex(input);
    assert_eq!(parsed_parts, expected_parts);
    assert_eq!(parsed_symbols, expected_symbols);

    let (parsed_parts, parsed_symbols) = parse_no_regex(input);
    assert_eq!(parsed_parts, expected_parts);
    assert_eq!(parsed_symbols, expected_symbols);
}
