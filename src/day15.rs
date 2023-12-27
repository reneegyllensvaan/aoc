static INPUT_FILE: &str = "input/day15";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

fn hash(input: &[u8]) -> u8 {
    let mut result = 0u32;
    for c in input.iter() {
        result += *c as u32;
        result *= 17;
        result = result % 256;
    }
    result as u8
}

pub fn part1(input: &str) -> i64 {
    let input = input.trim();
    let mut result = 0;
    for s in input.as_bytes().split(|c| *c == b',') {
        result += hash(s) as i64;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let input = input.trim();
    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![vec![]; 256];
    for expr in input.as_bytes().split(|c| *c == b',') {
        let id = expr.split(|c| *c == b'=' || *c == b'-').next().unwrap();
        let [op, n @ ..] = &expr[id.len()..] else {
            panic!("missing operation")
        };
        let h = hash(id) as usize;
        if *op == b'-' {
            boxes[h].retain(|v| v.0 != id);
        } else {
            let n: u8 = n[0] - b'0';
            if let Some(b) = boxes[h].iter_mut().find(|v| v.0 == id) {
                b.1 = n;
            } else {
                boxes[h].push((id, n));
            }
        }
    }

    let mut result = 0i64;
    for (b, box_num) in boxes.iter().zip(1..) {
        for ((_, foc), slot) in b.iter().zip(1..) {
            result += box_num * slot * *foc as i64;
        }
    }
    result
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
fn test_hash() {
    assert_eq!(hash(b"HASH"), 52);
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 1320);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 145);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 506437);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 288521);
}
