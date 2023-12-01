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

            if first.is_none() {
                first = Some(digit);
            }
            last = Some(digit);
        }
        match (first, last) {
            (Some(a), Some(b)) => result += a * 10 + b,
            _ => {}
        }
    }
    result
}

fn part2(input: &str) -> i64 {
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
            if first.is_none() {
                first = Some(digit);
            }
            last = Some(digit);
        }
        let to_add = match (first, last) {
            (Some(a), Some(b)) => a * 10 + b,
            _ => 0,
        };
        result += to_add;
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
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
    assert_eq!(part2(input), 281);
}
#[test]
fn example_part2_overlapping() {
    let input = r#"eightwo"#;
    assert_eq!(part2(input), 82);
}
