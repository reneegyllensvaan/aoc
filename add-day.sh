day=$(printf 'day%02d' $1)

run=$(sed -E 's/^(\s*)(.*\[NEXT DAY\])/        ("'$day'", aoc23::'$day'::main),\n\1\2/' src/run.rs)
echo "$run" > src/run.rs

echo "pub mod $day;" >> src/lib.rs

cat >src/$day.rs << EOF
static INPUT_FILE: &str = "input/$day";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"FIXME"#;

pub fn part1(input: &str) -> i64 {
    todo!()
}

pub fn part2(input: &str) -> i64 {
    todo!()
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1), ("part2", part2)];

    for (name, f) in &fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
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

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 46);
}

// #[test]
// fn test_part2_example() {
//     assert_eq!(part2(EXAMPLE_INPUT), 0);
// }

// #[test]
// fn test_part1_facit() {
//     let input = std::fs::read_to_string(INPUT_FILE).unwrap();
//     assert_eq!(part1(&input), 0);
// }

// #[test]
// fn test_part2_facit() {
//     let input = std::fs::read_to_string(INPUT_FILE).unwrap();
//     assert_eq!(part2(&input), 0);
// }
