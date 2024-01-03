use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use utils::{Grid, Pos, PosUtils, SGrid, Vec2dUtils};

static INPUT_FILE: &str = "input/day23";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

pub fn part1(input: &str) -> i64 {
    let mut grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();
    let end = (
        grid.len() - 1,
        grid.last()
            .unwrap()
            .iter()
            .find_position(|c| **c == '.')
            .unwrap()
            .0,
    );

    fn walk(
        pos: Pos,
        grid: &SGrid<char>,
        stepped: &mut HashSet<Pos>,
        visited: &mut HashSet<Pos>,
        len: usize,
        end: Pos,
    ) -> Option<usize> {
        visited.insert(pos);
        if pos == end {
            return Some(len);
        }
        if stepped.contains(&pos) {
            return None;
        }
        let mut result: Option<usize> = None;
        stepped.insert(pos);
        for neigh in pos.neighbors_in(grid) {
            let c = grid.at(neigh);
            let can_move = c == '.'
                || (c == '>' && neigh.1 > pos.1)
                || (c == '<' && neigh.1 < pos.1)
                || (c == 'v' && neigh.0 > pos.0);
            if can_move {
                if let Some(r) = walk(neigh, grid, stepped, visited, len + 1, end) {
                    result = Some(result.unwrap_or(r).max(r));
                }
            }
        }
        stepped.remove(&pos);
        result
    }

    let mut stepped = HashSet::<Pos>::new();
    let mut visited = HashSet::<Pos>::new();
    let result = walk((0, 1), &grid, &mut stepped, &mut visited, 0, end);
    for pos in visited {
        grid[pos.0][pos.1] = 'O';
    }
    // println!(
    //     "walked: \n{}",
    //     grid.into_iter().map(|l| l.into_iter().join("")).join("\n")
    // );

    result.unwrap() as i64
}

// pub fn part2(input: &str) -> i64 {
//     let mut grid: Grid<char> = input
//         .lines()
//         .map(|l| {
//             l.chars()
//                 .map(|v| if v == '>' || v == 'v' { '.' } else { v })
//                 .collect()
//         })
//         .collect();
//     let end = (
//         grid.len() - 1,
//         grid.last()
//             .unwrap()
//             .iter()
//             .find_position(|c| **c == '.')
//             .unwrap()
//             .0,
//     );
//     fn collect_edges(
//         from: Pos,
//         orig: Pos,
//         grid: &SGrid<char>,
//         edges: &mut Vec<(Pos, Pos, usize)>,
//         visited: &mut HashSet<Pos>,
//         seen: &mut HashSet<(Pos, Pos)>,
//         end: Pos,
//     ) {
//         let mut pos = orig;
//         let mut stepped_tiles = Vec::new();
//         if !seen.insert((from, orig)) {
//             return;
//         }
//         for steps in 0.. {
//             let spos = pos;
//             visited.insert(spos);
//             stepped_tiles.push(spos);
//             let mut neighs = pos.neighbors_in(&grid);
//             neighs.retain(|n| grid.at(*n) == '.' && !visited.contains(&n));
//             if neighs.is_empty() {
//                 if pos == end {
//                     edges.push((from, pos, steps));
//                     edges.push((pos, from, steps));
//                 } else {
//                     // println!(
//                     //     "edges: \n{}",
//                     //     edges.iter().map(|v| format!("{v:?}")).join("\n")
//                     // );
//                     println!("no neighs: {:?}", (from, orig, pos, steps));
//                 }
//                 break;
//             }
//             if let [v] = neighs.as_slice() {
//                 pos = *v;
//             } else {
//                 edges.push((from, pos, steps + 1));
//                 edges.push((pos, from, steps + 1));
//                 for n in neighs {
//                     collect_edges(pos, n, grid, edges, visited, seen, end);
//                 }
//                 break;
//             }
//         }
//         for p in stepped_tiles {
//             visited.remove(&p);
//         }
//     }
//     fn walk(
//         pos: Pos,
//         graph: &[(Pos, Pos, usize)],
//         stepped: &mut HashSet<Pos>,
//         len: usize,
//         end: Pos,
//     ) -> Option<usize> {
//         // println!("pos: {:?}", pos);
//         if pos == end {
//             // println!("pos == end: {:?}", pos);
//             return Some(len);
//         }
//         if stepped.contains(&pos) {
//             // println!("stepped contains: {:?}: {:?}", pos, stepped);
//             return None;
//         }
//         let mut result: Option<usize> = None;
//         stepped.insert(pos);
//         for (_, tar, step_len) in graph.iter().filter(|v| v.0 == pos) {
//             // println!("(pos, tar, step_len): {:?}", (pos, tar, step_len));
//             if let Some(r) = walk(*tar, graph, stepped, len + step_len, end) {
//                 // println!("r: {:?}", r);
//                 result = Some(result.unwrap_or(r).max(r));
//             }
//         }
//         stepped.remove(&pos);
//         result
//     }
//     let mut visited = HashSet::<Pos>::new();
//     let mut edges = Vec::<(Pos, Pos, usize)>::new();
//     collect_edges(
//         (0, 1),
//         (0, 1),
//         &grid,
//         &mut edges,
//         &mut visited,
//         &mut HashSet::new(),
//         end,
//     );
//     println!(
//         "edges: \n{}",
//         edges.iter().map(|v| format!("{v:?}")).join("\n")
//     );
//     println!("end: {:?}", end);
//     let result = walk((0, 1), &edges, &mut HashSet::new(), 0, end);
//     println!("result: {:?}", result);
//     // for pos in visited {
//     //     grid[pos.0][pos.1] = 'O';
//     // }
//     // println!(
//     //     "walked: \n{}",
//     //     grid.into_iter().map(|l| l.into_iter().join("")).join("\n")
//     // );
//     result.unwrap() as i64
// }

pub fn part2(input: &str) -> i64 {
    fn remove_slope(v: char) -> char {
        if v == '>' || v == 'v' {
            '.'
        } else {
            v
        }
    }

    let grid: Grid<char> = input
        .lines()
        .map(|l| l.chars().map(remove_slope).collect())
        .collect();
    let end = (
        grid.len() - 1,
        grid.last()
            .unwrap()
            .iter()
            .find_position(|c| **c == '.')
            .unwrap()
            .0,
    );

    let mut intersections = HashSet::<Pos>::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate().filter(|v| *v.1 == '.') {
            let pos: Pos = (x, y);
            let mut neigh = pos.neighbors_in(&grid);
            neigh.retain(|n| grid.at(*n) == '.');
            if neigh.len() > 2 {
                intersections.insert(pos);
            }
        }
    }
    intersections.insert((0, 1));
    intersections.insert(end);

    let mut edges: HashMap<Pos, Vec<(Pos, usize)>> = HashMap::new();
    for intersection in intersections.iter() {
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back((*intersection, 0usize));
        while let Some((pos, len)) = q.pop_front() {
            if !seen.insert(pos) {
                continue;
            }
            for neigh in pos
                .neighbors_in(&grid)
                .iter()
                .filter(|v| grid.at(**v) == '.')
            {
                if intersections.contains(neigh) && neigh != intersection {
                    edges
                        .entry(*intersection)
                        .or_default()
                        .push((*neigh, len + 1));
                    continue;
                } else {
                    q.push_back((*neigh, len + 1));
                }
            }
        }
    }

    let mut result = 0;
    // TODO: would be nice to not have to clone the prior path all the time? can we push a seen
    // value to the stack instead and maintain a hashset? maybe we should just recurse instead
    let mut q = Vec::<(Pos, usize, Vec<Pos>)>::new();
    q.push(((0, 1), 0, vec![]));
    while let Some((pos, len, seen)) = q.pop() {
        if pos == end {
            result = result.max(len as i64);
            continue;
        } else {
            let targets = edges.get(&pos).unwrap();
            for (target, edge_len) in targets {
                if !seen.contains(target) {
                    let mut s = seen.clone();
                    s.push(pos);
                    q.push((*target, len + edge_len, s));
                }
            }
        }
    }

    result
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 1;

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
    assert_eq!(part1(EXAMPLE_INPUT), 94);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 154);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 2278);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 6734);
}
