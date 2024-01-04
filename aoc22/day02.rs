static INPUT_FILE: &str = "input/day02";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"A Y
B X
C Z"#;

pub fn part1(input: &str) -> i64 {
    let moves = input.lines().map(|l| l.split_once(' ').unwrap());
    let mut result = 0;
    for (they, me) in moves {
        let they = match they {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            v => panic!("invalid move: {v:?}"),
        };
        let me = match me {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            v => panic!("invalid move: {v:?}"),
        };
        result += me + 1;
        result += match (me, they) {
            (a, b) if a == b => 3,
            (0, 2) => 6,
            (1, 0) => 6,
            (2, 1) => 6,
            _ => 0,
        };
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let moves = input.lines().map(|l| l.split_once(' ').unwrap());
    let mut result = 0i64;
    for (they, strat) in moves {
        let they: i64 = match they {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            v => panic!("invalid move: {v:?}"),
        };
        result += match strat {
            "Y" => 3,
            "X" => 0,
            "Z" => 6,
            v => panic!("invalid move: {v:?}"),
        };
        let me = match strat {
            "Y" => they,
            "X" => (they + 2) % 3,
            "Z" => (they + 4) % 3,
            v => panic!("invalid move: {v:?}"),
        };
        result += me + 1;
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
    assert_eq!(part1(EXAMPLE_INPUT), 15);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 12);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 11906);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 11186);
}
