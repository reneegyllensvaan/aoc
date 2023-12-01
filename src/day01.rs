/// Version 1.
///
/// This one's pretty trivial, nothing fun to optimize.
///
/// Notably, this works with UTF-8.
///
/// Benchmark on my computer: around 41us/iter (522MB/s)
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

/// Version 1 of part 2.
///
/// This one still works with UTF-8 strings.
///
/// I use a regex here, which is kinda boring imo.
///
/// Benchmark on my computer: around 659us/iter (33MB/s)
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

/// General non-regex approach for finding digits within a slice.
fn get_digit(at: &[u8]) -> Option<i64> {
    if let Some(c @ b'1'..=b'9') = at.first() {
        Some(-(b'0' as i16 - *c as i16) as i64)
    } else {
        Some(match at {
            [b'o', b'n', b'e', ..] => 1,
            [b't', b'w', b'o', ..] => 2,
            [b't', b'h', b'r', b'e', b'e', ..] => 3,
            [b'f', b'o', b'u', b'r', ..] => 4,
            [b'f', b'i', b'v', b'e', ..] => 5,
            [b's', b'i', b'x', ..] => 6,
            [b's', b'e', b'v', b'e', b'n', ..] => 7,
            [b'e', b'i', b'g', b'h', b't', ..] => 8,
            [b'n', b'i', b'n', b'e', ..] => 9,
            _ => return None,
        })
    }
}

/// Version 2.
///
/// Here I stop using a regex, and stop handling utf-8.
///
/// Benchmark on my computer: around 123us/iter (174MB/s)
fn part2_no_regex(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines().map(str::as_bytes) {
        let mut first = None;
        let mut last = None;
        for (ix, _) in line.iter().enumerate() {
            let Some(digit) = get_digit(&line[ix..]) else {
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

/// Version 3.
///
/// Searches twice - once in each direction.
///
/// For a line like `1aaaaaaaaaaaaaaaa2`, you don't need to look at the whole string, only start
/// scanning from the start until you find the first digit, and then from the end until you find a
/// digit.
///
/// Benchmark on my computer: around 55us/iter (390MB/s)
fn part2_no_regex_bidir(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines().map(str::as_bytes) {
        let mut first = None;
        let mut last = None;
        for ix in 0..line.len() {
            let Some(digit) = get_digit(&line[ix..]) else {
                continue;
            };
            first = Some(digit);
            break;
        }

        for ix in (0..line.len()).rev() {
            let Some(digit) = get_digit(&line[ix..]) else {
                continue;
            };
            last = Some(digit);
            break;
        }
        if let (Some(a), Some(b)) = (first, last) {
            result += a * 10 + b;
        };
    }
    result
}

/// Version 4.
///
/// When searching bidirectionally, we don't actually need the first and last values at the same
/// time, so we don't need variables for them - just add them to the result and break. This gets
/// rid of some stack space and branches.
///
/// Benchmark on my computer: around 50us/iter (428MB/s)
fn part2_no_regex_bidir_add_directly(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines().map(str::as_bytes) {
        for ix in 0..line.len() {
            if let Some(digit) = get_digit(&line[ix..]) {
                result += 10 * digit;
                break;
            };
        }

        for ix in (0..line.len()).rev() {
            if let Some(digit) = get_digit(&line[ix..]) {
                result += digit;
                break;
            };
        }
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2 (regex):    {}", part2_regex(&input));
    println!("part2 (no regex): {}", part2_no_regex(&input));
    println!("part2 (no regex, bidir): {}", part2_no_regex_bidir(&input));
    println!(
        "part2 (no regex, bidir, add directly): {}",
        part2_no_regex_bidir_add_directly(&input)
    );

    let begin = std::time::Instant::now();
    for _ in 0..10000 {
        part1(&input);
    }
    let end = std::time::Instant::now();
    println!(
        "10k part1 in: {}us ({}us/iter)",
        (end - begin).as_micros(),
        (end - begin).as_micros() / 10000
    );

    let begin = std::time::Instant::now();
    for _ in 0..10000 {
        part2_regex(&input);
    }
    let end = std::time::Instant::now();
    println!(
        "10k part2 (regex) in: {}us ({}us/iter)",
        (end - begin).as_micros(),
        (end - begin).as_micros() / 10000
    );

    let begin = std::time::Instant::now();
    for _ in 0..10000 {
        part2_no_regex(&input);
    }
    let end = std::time::Instant::now();
    println!(
        "10k part2 (no regex) in: {}us ({}us/iter)",
        (end - begin).as_micros(),
        (end - begin).as_micros() / 10000
    );

    let begin = std::time::Instant::now();
    for _ in 0..10000 {
        part2_no_regex_bidir(&input);
    }
    let end = std::time::Instant::now();
    println!(
        "10k part2 (no regex, bidir) in: {}us ({}us/iter)",
        (end - begin).as_micros(),
        (end - begin).as_micros() / 10000
    );

    let begin = std::time::Instant::now();
    for _ in 0..10000 {
        part2_no_regex_bidir_add_directly(&input);
    }
    let end = std::time::Instant::now();
    println!(
        "10k part2 (no regex, bidir, add directly) in: {}us ({}us/iter)",
        (end - begin).as_micros(),
        (end - begin).as_micros() / 10000
    );
}

#[test]
fn test_facit_part1() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    assert_eq!(part1(&input), 55712);
}

#[test]
fn test_facit_part2() {
    let input = std::fs::read_to_string("./input/day01").unwrap();
    assert_eq!(part2_regex(&input), 55413);
    assert_eq!(part2_no_regex(&input), 55413);
    assert_eq!(part2_no_regex_bidir(&input), 55413);
    assert_eq!(part2_no_regex_bidir_add_directly(&input), 55413);
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
    assert_eq!(part2_regex(input), 281);
    assert_eq!(part2_no_regex(input), 281);
    assert_eq!(part2_no_regex_bidir(input), 281);
    assert_eq!(part2_no_regex_bidir_add_directly(input), 281);
}
#[test]
fn example_part2_overlapping() {
    let input = r#"eightwo"#;
    assert_eq!(part2_regex(input), 82);
    assert_eq!(part2_no_regex(input), 82);
    assert_eq!(part2_no_regex_bidir(input), 82);
    assert_eq!(part2_no_regex_bidir_add_directly(input), 82);
}
