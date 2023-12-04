fn part1(input: &str) -> i64 {
    let mut result = 0;
    'line: for line in input.lines() {
        if line.len() < 5 {
            // i have some blank lines in my tests
            continue;
        }
        let Some((game_id, tail)) = line.trim_start()[5..].split_once(':') else {
            continue;
        };
        let game_id: i64 = game_id.parse().unwrap();
        for round in tail.split(';') {
            for cubes in round.split(',').map(str::trim) {
                let (num, color) = cubes.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                let possible = match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    _ => panic!("{color}"),
                };
                if !possible {
                    continue 'line;
                }
            }
        }
        result += game_id;
    }
    result
}

fn part2(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        if line.len() < 5 {
            // i have some blank lines in my tests
            continue;
        }
        let Some((_, tail)) = line.trim_start()[5..].split_once(':') else {
            continue;
        };
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in tail.split(';') {
            for cubes in round.split(',').map(str::trim) {
                let (num, color) = cubes.split_once(' ').unwrap();
                let num: i64 = num.parse().unwrap();
                match color {
                    "red" => {
                        red = red.max(num);
                    }
                    "green" => {
                        green = green.max(num);
                    }
                    "blue" => {
                        blue = blue.max(num);
                    }
                    _ => panic!("{color}"),
                };
            }
        }
        result += red * blue * green;
    }
    result
}

pub fn main() {
    let input = std::fs::read_to_string("src/day02").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_part1_example() {
    let input = r#"
  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
  Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
  Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
  Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
  "#;
    assert_eq!(part1(input), 8);
}
#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day02").unwrap();
    assert_eq!(part1(&input), 2317);
}

#[test]
fn test_part2_example() {
    let input = r#"
  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
  Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
  Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
  Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
  "#;
    assert_eq!(part2(input), 2286);
}
#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day02").unwrap();
    assert_eq!(part2(&input), 74804);
}
