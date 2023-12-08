use std::cmp::Reverse;

use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let mut games = input
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|v| v.tuple_windows().next().unwrap())
        .map(|(a, b)| {
            (
                into_card_powers(a.as_bytes(), false),
                b.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    games.sort_by_key(|(g, _)| (match_type(g), g.clone()));

    games
        .iter()
        .zip(1..)
        .map(|((_, score), rank)| score * rank)
        .sum()
}

fn into_card_powers(input: &[u8], jokers: bool) -> Vec<u8> {
    input
        .iter()
        .map(|c| match c {
            b'2'..=b'9' => 2 + c - b'2',
            b'T' => 10,
            b'J' => {
                if jokers {
                    0
                } else {
                    11
                }
            }
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => panic!("unexpected card: <{c}> for line ({input:?})"),
        })
        .collect()
}

pub fn part2(input: &str) -> i64 {
    let mut games = input
        .split_whitespace()
        .chunks(2)
        .into_iter()
        .map(|v| v.tuple_windows().next().unwrap())
        .map(|(a, b)| {
            (
                into_card_powers(a.as_bytes(), true),
                b.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    games.sort_by_key(|(g, _)| (match_type(g), g.clone()));

    games
        .iter()
        .zip(1..)
        .map(|((_, score), rank)| score * rank)
        .sum()
}
fn match_type(v: &[u8]) -> i64 {
    let mut s = v.to_owned();
    s.sort_unstable();
    let mut a = s
        .iter()
        .filter(|c| **c != 0)
        .dedup_with_count()
        .map(|v| v.0)
        .collect_vec();
    a.sort_unstable_by_key(|c| Reverse(*c));
    match &a[..] {
        // upgrade to 5 of a kind
        [1..=4] | [] => 7,

        // upgrade to 4 of a kind
        [1..=3, 1] => 6,

        // upgrade to full house
        [2, 2] => 5,

        // upgrade to two pair
        [1 | 2, 1, 1] => 4,

        // upgrade to pair
        [1, 1, 1, 1] => 2,

        // no jokers, handle like normal hand
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        [3, 1, 1] => 4,
        [2, 2, 1] => 3,
        [2, 1, 1, 1] => 2,
        [1, 1, 1, 1, 1] => 1,
        e => panic!("invalid counts '{e:?}' for line {v:?}"),
    }
}

/// Version 2.
///
/// Sorting was taking up a heck of a time, because i ran [`match_type`] potentially lots of times.
/// `match_type` is sort of expensive because it does both allocations and sorting.
///
/// Solved by just running it once in the iterator and sorting with it allocated.
///
/// Runs in about 220us/iter for me - I'm happy enough with that.
///
/// Further optimization might mean I have to break up my pretty little iterator chain, and we
/// can't have that, can we?
pub fn solve_faster(input: &str, jokers: bool) -> i64 {
    input
        .lines()
        .flat_map(|v| v.split_once(' '))
        .map(|(a, b)| {
            (
                into_card_powers(a.as_bytes(), jokers),
                b.parse::<i64>().unwrap(),
            )
        })
        .map(|(g, bid)| (match_type(&g), g, bid))
        .sorted_unstable()
        .zip(1..)
        .map(|((_, _, bid), score)| score * bid)
        .sum()
}
pub fn part1_faster(input: &str) -> i64 {
    solve_faster(input, false)
}
pub fn part2_faster(input: &str) -> i64 {
    solve_faster(input, true)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07").unwrap();

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 4] = [
        ("part1", part1),
        ("part1 (faster)", part1_faster),
        ("part2", part2),
        ("part2 (faster)", part2_faster),
    ];

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
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part1(input), 6440);
    assert_eq!(part1_faster(input), 6440);
}

#[test]
fn test_part2_example() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part2(input), 5905);
    assert_eq!(part2_faster(input), 5905);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day07").unwrap();
    assert_eq!(part1(&input), 250058342);
    assert_eq!(part1_faster(&input), 250058342);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day07").unwrap();
    assert_eq!(part2(&input), 250506580);
    assert_eq!(part2_faster(&input), 250506580);
}

#[test]
fn test_match_with_jokers() {
    assert_eq!(2, match_type(&into_card_powers(b"32T3K", true)));
    assert_eq!(6, match_type(&into_card_powers(b"T55J5", true)));
    assert_eq!(3, match_type(&into_card_powers(b"KK677", true)));
    assert_eq!(6, match_type(&into_card_powers(b"KTJJT", true)));
    assert_eq!(6, match_type(&into_card_powers(b"QQQJA", true)));
    assert_eq!(7, match_type(&into_card_powers(b"JJJJJ", true)));
    assert_eq!(7, match_type(&into_card_powers(b"AJJJJ", true)));
    assert_eq!(7, match_type(&into_card_powers(b"AJAJJ", true)));
    assert_eq!(7, match_type(&into_card_powers(b"AJAJA", true)));
    assert_eq!(4, match_type(&into_card_powers(b"234JJ", true)));
    assert_eq!(6, match_type(&into_card_powers(b"23JJJ", true)));
}
