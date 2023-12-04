use std::collections::HashSet;

fn part1(input: &str) -> i64 {
    let mut result: i64 = 0;

    for line in input.lines() {
        let (card_id, rest) = line.split_once(':').unwrap();
        let _card_id: i64 = card_id
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let (numbers, guesses) = rest.split_once('|').unwrap();
        let numbers: Vec<i64> = numbers
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let guesses: Vec<i64> = guesses
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let mut line_score = 0;
        for number in &numbers {
            for guess in &guesses {
                if number == guess {
                    line_score = 1.max(line_score * 2);
                    break;
                }
            }
        }
        result += line_score;
    }

    result
}

fn part2(input: &str) -> i64 {
    let mut result: i64 = 0;

    let mut copies = Vec::<(usize, i64)>::new();
    for (ix, line) in input.lines().enumerate() {
        let (_card_id, rest) = line.split_once(':').unwrap();

        let (numbers, guesses) = rest.split_once('|').unwrap();
        let numbers: Vec<i64> = numbers
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let guesses: Vec<i64> = guesses
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let num_copies: i64 = 1 + copies
            .iter()
            .filter(|v| ix <= v.0)
            .map(|v| v.1)
            .sum::<i64>();
        result += num_copies;
        let mut line_matches = 0;
        for number in &numbers {
            for guess in &guesses {
                if number == guess {
                    line_matches += 1;
                    break;
                }
            }
        }
        if line_matches > 0 {
            copies.push((ix + line_matches, num_copies));
        }
    }

    result
}
fn part2_hash_set(input: &str) -> i64 {
    let mut result: i64 = 0;

    let mut copies = Vec::<(usize, i64)>::new();
    for (ix, line) in input.lines().enumerate() {
        let (_card_id, rest) = line.split_once(':').unwrap();

        let (numbers, guesses) = rest.split_once('|').unwrap();
        let numbers: Vec<i64> = numbers
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let guesses: HashSet<i64> = guesses
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();

        let num_copies: i64 = 1 + copies
            .iter()
            .filter(|v| ix <= v.0)
            .map(|v| v.1)
            .sum::<i64>();
        result += num_copies;
        let mut line_matches = 0;
        for number in &numbers {
            if guesses.contains(number) {
                line_matches += 1;
                break;
            }
        }
        if line_matches > 0 {
            copies.push((ix + line_matches, num_copies));
        }
    }

    result
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04").unwrap();

    let iters = 10;

    let fns: [(&'static str, fn(&str) -> i64); 3] = [
        ("part1", part1),
        ("part2", part2),
        ("part2 (hash set)", part2_hash_set),
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
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part1(input), 13);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day04").unwrap();
    assert_eq!(part1(&input), 27059);
}

#[test]
fn test_part2_example() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part2(input), 30);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day04").unwrap();
    assert_eq!(part2(&input), 5744979);
}
