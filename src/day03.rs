use std::ops::Range;

#[derive(Debug, Clone)]
struct PartNumber {
  num: i32,
  row: i32,
  span: Range<i32>,
}

fn part1(input: &str) -> i32 {
  let mut result: i32 = 0;

  let mut parts: Vec<PartNumber> = Vec::new();
  let mut symbols: Vec<(i32, i32)> = Vec::new();

  let expr = regex::Regex::new(r"[0-9]+|[^0-9.]").unwrap();
  for (row, line) in input.lines().enumerate() {
    for m in expr.find_iter(line) {
      if let Ok(num) = m.as_str().parse::<i32>() {
        parts.push(PartNumber { 
          num, 
          row: row as i32, 
          span: (m.start() as i32)..(m.end() as i32)
        });
      } else {
        symbols.push((row as i32, m.start() as i32));
      }
    }
  }

  for part in parts.iter() {
    for symbol in symbols.iter() {
      let within_row = part.row >= symbol.0-1 && part.row <= symbol.0+1;
      let within_span = symbol.1 >= part.span.start-1 && symbol.1 <= part.span.end;
      if within_row && within_span {
        result += part.num;
        break;
      }
    }
  }
  result
}
fn part2(input: &str) -> i32 {
  let mut result: i32 = 0;

  let mut parts: Vec<PartNumber> = Vec::new();
  let mut symbols: Vec<(i32, i32)> = Vec::new();

  let expr = regex::Regex::new(r"[0-9]+|[*]").unwrap();
  for (row, line) in input.lines().enumerate() {
    for m in expr.find_iter(line) {
      if let Ok(num) = m.as_str().parse::<i32>() {
        parts.push(PartNumber { 
          num, 
          row: row as i32, 
          span: (m.start() as i32)..(m.end() as i32)
        });
      } else {
        symbols.push((row as i32, m.start() as i32));
      }
    }
  }

  for symbol in symbols.iter() {
    let mut adj: Vec<PartNumber> = Vec::new();
    for part in parts.iter() {
      let within_row = part.row >= symbol.0-1 && part.row <= symbol.0+1;
      let within_span = symbol.1 >= part.span.start-1 && symbol.1 <= part.span.end;
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

fn main() {
  let input = std::fs::read_to_string("day03").unwrap();
  println!("part1: {}", part1(&input));
  println!("part2: {}", part2(&input));
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
}

#[test]
fn test_part1_facit() {
  let input = std::fs::read_to_string("input/day03").unwrap();
  assert_eq!(part1(&input), 536576);
}

#[test]
fn test_part2_facit() {
  let input = std::fs::read_to_string("input/day03").unwrap();
  assert_eq!(part2(&input), 75741499);
}
