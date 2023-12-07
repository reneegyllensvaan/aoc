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

    for (game, _) in games.iter() {
        println!("game={:?} rank={}", game, match_type(game));
    }

    games
        .iter()
        .inspect(|g| println!("g: {g:?}"))
        .zip(1..)
        .map(|((_, score), rank)| score * rank)
        .sum()
}
fn match_type(v: &[u8]) -> i64 {
    let jokers = v.iter().filter(|c| **c == 0).count() as i64;
    let mut s = v.to_owned();
    s.sort_unstable();
    let mut a = s
        .iter()
        .filter(|c| **c != 0)
        .dedup_with_count()
        .map(|(c, _)| (c as i64))
        .collect_vec();
    a.sort_unstable_by_key(|c| -c);
    match &a[..] {
        [4, ..] if jokers >= 1 => 7,
        [3, ..] if jokers >= 1 => 5 + jokers,
        [2, ..] if jokers == 2 => 6,
        [2, ..] if jokers == 3 => 7,
        [2, 2] if jokers == 1 => 5,
        [] => 7,
        [1] => 7,
        [2, 1, 1] => 4,
        [1, 1] => 6,
        [1, 1, 1] => 4,
        [1, 1, 1, 1] => 2,

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

pub fn main() {
    let input = std::fs::read_to_string("input/day07").unwrap();

    let iters = 1000;

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
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part1(input), 6440);
}

#[test]
fn test_part2_example() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(part2(input), 5905);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day07").unwrap();
    assert_eq!(part1(&input), 250058342);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day07").unwrap();
    assert_eq!(part2(&input), 250506580);
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
