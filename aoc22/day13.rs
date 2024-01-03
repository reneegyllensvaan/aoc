static INPUT_FILE: &str = "input/day13";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"FIXME"#;

pub fn part1(input: &str) -> i64 {
    let _ = input;
    todo!()
}

pub fn part2(input: &str) -> i64 {
    let _ = input;
    todo!()
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
