use crate::utils::Vec2dUtils;

static INPUT_FILE: &str = "input/day13";

pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    let mut section = Vec::<Vec<char>>::new();
    for line in input.lines() {
        if line.is_empty() {
            let v = find_mirror(&section);
            result += v;
            section.clear();
        } else {
            section.push(line.chars().collect());
        }
    }
    result += find_mirror(&section);
    result
}
pub fn part1_iters(input: &str) -> i64 {
    let mut result = 0;
    let mut section = Vec::<Vec<char>>::new();
    for line in input.lines() {
        if line.is_empty() {
            let v = find_mirror_iters(&section);
            result += v;
            section.clear();
        } else {
            section.push(line.chars().collect());
        }
    }
    result += find_mirror_iters(&section);
    result
}

fn find_mirror(section: &[Vec<char>]) -> i64 {
    find_mirror_transpose(section)
}

/// Find mirror point using transposition to get the alternate axis
fn find_mirror_transpose(section: &[Vec<char>]) -> i64 {
    let col_count = section[0].len();

    'col: for ix in 1..col_count {
        let len = ix.min(col_count - ix);
        for line in section {
            let l = &line[(ix - len)..ix];
            let r = &line[ix..(ix + len)];
            if !l.iter().zip(r.iter().rev()).all(|(a, b)| a == b) {
                continue 'col;
            }
        }
        return ix as i64;
    }

    let section = section.transpose();
    let col_count = section[0].len();

    'col: for ix in 1..col_count {
        let len = ix.min(col_count - ix);
        for line in &section {
            let l = &line[(ix - len)..ix];
            let r = &line[ix..(ix + len)];
            if !l.iter().zip(r.iter().rev()).all(|(a, b)| a == b) {
                continue 'col;
            }
        }
        return (ix as i64) * 100;
    }

    panic!("no mirror found for section:\n{section:?}");
}
fn find_mirror_iters(section: &[Vec<char>]) -> i64 {
    let col_count = section[0].len();
    let row_count = section.len();

    'col: for ix in 1..col_count {
        let len = ix.min(col_count - ix);
        for row_ix in 0..row_count {
            let l = (ix - len)..ix;
            let r = ix..(ix + len);
            for (l_ix, r_ix) in l.zip(r.rev()) {
                let l_c = section[row_ix][l_ix];
                let r_c = section[row_ix][r_ix];
                if l_c != r_c {
                    continue 'col;
                }
            }
        }
        return ix as i64;
    }

    let row_count = section.len();
    'row: for ix in 1..row_count {
        let len = ix.min(row_count - ix);
        for col_ix in 0..col_count {
            let l = (ix - len)..ix;
            let r = ix..(ix + len);
            for (l_ix, r_ix) in l.zip(r.rev()) {
                let l_c = section[l_ix][col_ix];
                let r_c = section[r_ix][col_ix];
                if l_c != r_c {
                    continue 'row;
                }
            }
        }
        return (ix as i64) * 100;
    }

    panic!("no mirror found for section:\n{section:?}");
}

pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    let mut section = Vec::<Vec<char>>::new();
    for line in input.lines() {
        if line.is_empty() {
            let v = find_smudge_iters(&section);
            result += v;
            section.clear();
        } else {
            section.push(line.chars().collect());
        }
    }
    result += find_smudge_iters(&section);
    result
}
fn find_smudge_iters(section: &[Vec<char>]) -> i64 {
    let col_count = section[0].len();
    let row_count = section.len();

    for ix in 1..col_count {
        let len = ix.min(col_count - ix);
        let mut dev = 0;
        for row_ix in 0..row_count {
            let l = (ix - len)..ix;
            let r = ix..(ix + len);
            for (l_ix, r_ix) in l.zip(r.rev()) {
                let l_c = section[row_ix][l_ix];
                let r_c = section[row_ix][r_ix];
                if l_c != r_c {
                    dev += 1;
                }
            }
        }
        if dev == 1 {
            return ix as i64;
        }
    }

    let row_count = section.len();
    for ix in 1..row_count {
        let len = ix.min(row_count - ix);
        let mut dev = 0;
        for col_ix in 0..col_count {
            let l = (ix - len)..ix;
            let r = ix..(ix + len);
            for (l_ix, r_ix) in l.zip(r.rev()) {
                let l_c = section[l_ix][col_ix];
                let r_c = section[r_ix][col_ix];
                if l_c != r_c {
                    dev += 1;
                }
            }
        }
        if dev == 1 {
            return (ix as i64) * 100;
        }
    }

    panic!("no mirror found for section:\n{section:?}");
    // 0
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 100;

    let fns: [(&'static str, fn(&str) -> i64); 3] = [
        ("part1", part1),
        ("part1_iters", part1_iters),
        ("part2", part2),
    ];

    for (name, f) in fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
        for (name, f) in fns {
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
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part1(input), 405);
}

#[test]
fn test_part2_example() {
    let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!(part2(input), 400);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 27505);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 22906);
}
