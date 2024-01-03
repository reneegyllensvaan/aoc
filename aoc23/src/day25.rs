use std::collections::{HashMap, HashSet, VecDeque};

static INPUT_FILE: &str = "input/day25";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

pub fn part1(input: &str) -> i64 {
    let mut flat_edges = Vec::<(&str, &str)>::new();
    let mut edges = HashMap::<&str, HashSet<&str>>::new();
    let mut nodes = HashSet::<&str>::new();
    for line in input.lines() {
        let (src, tail) = line.split_once(": ").unwrap();
        nodes.insert(src);
        for tar in tail.split_whitespace() {
            edges.entry(src).or_default().insert(tar);
            edges.entry(tar).or_default().insert(src);
            flat_edges.push((src, tar));
            nodes.insert(tar);
            // println!("{src} --- {tar};");
        }
    }
    // return -1;

    // Ok, so. this obviously doesn't finish fast enough if you brute force it. you can supposedly
    // do some fancy algorithms. so what i did was just run it through graphviz. and u can very
    // clearly see three edges connecting two big clusters.
    // TODO: do the grownup version: https://en.wikipedia.org/wiki/Karger%27s_algorithm
    let edge_1 = ("vgk", "mbq");
    let edge_2 = ("nmv", "thl");
    let edge_3 = ("fzb", "fxr");
    edges.get_mut(edge_1.0).unwrap().remove(edge_1.1);
    edges.get_mut(edge_1.1).unwrap().remove(edge_1.0);
    edges.get_mut(edge_2.0).unwrap().remove(edge_2.1);
    edges.get_mut(edge_2.1).unwrap().remove(edge_2.0);
    edges.get_mut(edge_3.0).unwrap().remove(edge_3.1);
    edges.get_mut(edge_3.1).unwrap().remove(edge_3.0);

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(nodes.iter().next().unwrap());
    while let Some(node) = q.pop_front() {
        if !seen.insert(node) {
            continue;
        }
        for next in edges.get(node).into_iter().flatten() {
            q.push_back(next);
        }
    }
    let is_split = seen.len() != nodes.len();
    if is_split {
        // println!("edge_1: {:?}", edge_1);
        // println!("edge_2: {:?}", edge_2);
        // println!("edge_3: {:?}", edge_3);
        return (nodes.len() as i64 - seen.len() as i64) * seen.len() as i64;
    }

    panic!("graph is not split :(")
}

pub fn part2(input: &str) -> i64 {
    let _ = input;
    todo!()
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1)];

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
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 600369);
}
