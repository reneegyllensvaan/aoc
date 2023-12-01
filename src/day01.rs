fn part1(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            let Some(digit) = (match c {
                '0'..='9' => c.to_digit(10).map(|v| v as i64),
                _ => None,
            }) else {
                continue;
            };
            first.get_or_insert(digit);
            last = Some(digit);
        }
        if let (Some(a), Some(b)) = (first, last) {
            result += a * 10 + b;
        };
    }
    result
}

fn part2_regex(input: &str) -> i64 {
    let mut result: i64 = 0;
    let exp = regex::Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine|.*?").unwrap();
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        let mut end = 0;
        while end < line.len() {
            let Some(m) = exp.find(&line[end..]) else {
                continue;
            };
            end += 1;
            let digit = match (m.as_str().parse::<i64>(), m.as_str()) {
                (Ok(v), _) => v as i64,
                (_, "one") => 1,
                (_, "two") => 2,
                (_, "three") => 3,
                (_, "four") => 4,
                (_, "five") => 5,
                (_, "six") => 6,
                (_, "seven") => 7,
                (_, "eight") => 8,
                (_, "nine") => 9,
                _ => continue,
            };
            first.get_or_insert(digit);
            last = Some(digit);
        }
        if let (Some(a), Some(b)) = (first, last) {
            result += a * 10 + b;
        };
    }
    result
}

fn part2_no_regex(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines().map(str::as_bytes) {
        let mut first = None;
        let mut last = None;
        for (ix, c) in line.iter().enumerate() {
            let digit = if let b'1'..=b'9' = c {
                -(b'0' as i16 - *c as i16) as i64
            } else {
                match &line[ix..] {
                    [b'o', b'n', b'e', ..] => 1,
                    [b't', b'w', b'o', ..] => 2,
                    [b't', b'h', b'r', b'e', b'e', ..] => 3,
                    [b'f', b'o', b'u', b'r', ..] => 4,
                    [b'f', b'i', b'v', b'e', ..] => 5,
                    [b's', b'i', b'x', ..] => 6,
                    [b's', b'e', b'v', b'e', b'n', ..] => 7,
                    [b'e', b'i', b'g', b'h', b't', ..] => 8,
                    [b'n', b'i', b'n', b'e', ..] => 9,
                    _ => continue,
                }
            };
            first.get_or_insert(digit);
            last = Some(digit);
        }
        if let (Some(a), Some(b)) = (first, last) {
            result += a * 10 + b;
        };
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2 (regex):    {}", part2_regex(&input));
    println!("part2 (no regex): {}", part2_no_regex(&input));
}

#[test]
fn test_facit_part1() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    assert_eq!(part1(&input), 55712);
}

#[test]
fn test_facit_part2() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    assert_eq!(part2_no_regex(&input), 55413);
}

#[test]
fn example_part1() {
    let input = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "#;
    assert_eq!(part1(input), 142);
}

#[test]
fn example_part2() {
    let input = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "#;
    assert_eq!(part2_no_regex(input), 281);
}
#[test]
fn example_part2_overlapping() {
    let input = r#"eightwo"#;
    assert_eq!(part2_no_regex(input), 82);
}
