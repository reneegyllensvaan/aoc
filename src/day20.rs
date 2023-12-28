use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

static INPUT_FILE: &str = "input/day20";

pub fn part1(input: &str) -> i64 {
    let mut flip_flops = HashMap::<&str, (bool, Vec<&str>)>::new();
    let mut conjunctions = HashMap::<&str, (Vec<(&str, bool)>, Vec<&str>)>::new();
    for mut line in input.lines() {
        let t = line.chars().next().unwrap();
        if !line.starts_with("broadcaster") {
            line = &line[1..];
        }
        let (from, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").collect_vec();
        if t == '&' {
            conjunctions.insert(from, (vec![], targets));
        } else {
            flip_flops.insert(from, (false, targets));
        }
    }
    let mut deps: Vec<(&str, &str)> = vec![];
    // add conjunction backrefs
    for (from, flip) in flip_flops.iter() {
        for target in flip.1.iter() {
            deps.push((from, target));
        }
    }
    for (from, flip) in conjunctions.iter() {
        for target in flip.1.iter() {
            deps.push((from, target));
        }
    }
    for (src, dest) in deps {
        if let Some(v) = conjunctions.get_mut(dest) {
            v.0.push((src, false));
        }
    }

    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        let mut q = VecDeque::<(&str, bool, &str)>::new();
        q.push_back(("broadcaster", false, "button"));
        while let Some((id, sig, sender)) = q.pop_front() {
            if cfg!(test) {
                println!("{sender} -{}-> {id}", if sig { "high" } else { "low" });
            }
            if sig {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(flip_flop) = flip_flops.get_mut(id) {
                if id == "broadcaster" {
                    for target in flip_flop.1.iter() {
                        q.push_back((target, sig, id));
                    }
                } else {
                    // Flip-flop modules (prefix %) are either on or off; they are initially off. If a
                    // flip-flop module receives a high pulse, it is ignored and nothing happens. However,
                    // if a flip-flop module receives a low pulse, it flips between on and off. If it was
                    // off, it turns on and sends a high pulse. If it was on, it turns off and sends a low
                    // pulse.
                    if !sig {
                        flip_flop.0 ^= true;
                        for target in flip_flop.1.iter() {
                            q.push_back((target, flip_flop.0, id));
                        }
                    }
                }
            } else if let Some(conjunction) = conjunctions.get_mut(id) {
                // Conjunction modules (prefix &) remember the type of the most recent pulse
                // received from each of their connected input modules; they initially default to
                // remembering a low pulse for each input. When a pulse is received, the
                // conjunction module first updates its memory for that input. Then, if it
                // remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends
                // a high pulse.
                for input in conjunction.0.iter_mut() {
                    if input.0 == sender {
                        input.1 = sig;
                    }
                }
                let output = !conjunction.0.iter().all(|v| v.1);
                for target in conjunction.1.iter() {
                    q.push_back((target, output, id));
                }
            }
        }
    }

    highs * lows
}

pub fn part2(input: &str) -> i64 {
    let mut flip_flops = HashMap::<&str, (bool, Vec<&str>)>::new();
    let mut conjunctions = HashMap::<&str, (Vec<(&str, bool)>, Vec<&str>)>::new();
    for mut line in input.lines() {
        let t = line.chars().next().unwrap();
        if !line.starts_with("broadcaster") {
            line = &line[1..];
        }
        let (from, targets) = line.split_once(" -> ").unwrap();
        let targets = targets.split(", ").collect_vec();
        if t == '&' {
            conjunctions.insert(from, (vec![], targets));
        } else {
            flip_flops.insert(from, (false, targets));
        }
    }
    let mut deps: Vec<(&str, &str)> = vec![];
    // add conjunction backrefs
    for (from, flip) in flip_flops.iter() {
        for target in flip.1.iter() {
            deps.push((from, target));
        }
    }
    for (from, flip) in conjunctions.iter() {
        for target in flip.1.iter() {
            deps.push((from, target));
        }
    }
    for (src, dest) in deps {
        if let Some(v) = conjunctions.get_mut(dest) {
            v.0.push((src, false));
        }
    }

    // this isn't generic i suppose, but then neither is the lcm approach in general? dont like
    // this day, same as day08, the fact that the lcm approach works is an accident and it doesn't
    // work without it. allowing myself to hard-code input-dependent stuff bc of this lol.
    let mut rx_inputs: HashSet<&str> = conjunctions
        .iter()
        .filter(|v| v.1 .1.contains(&"rg"))
        .map(|v| *v.0)
        .collect();
    let mut cycle_lengths: Vec<i64> = vec![];
    'find_cycles: for ix in 1.. {
        let mut q = VecDeque::<(&str, bool, &str)>::new();
        q.push_back(("broadcaster", false, "button"));
        while let Some((id, sig, sender)) = q.pop_front() {
            if cfg!(test) {
                println!("{sender} -{}-> {id}", if sig { "high" } else { "low" });
            }

            if !sig {
                if rx_inputs.contains(id) {
                    cycle_lengths.push(ix);
                    rx_inputs.remove(id);
                    if rx_inputs.is_empty() {
                        break 'find_cycles;
                    }
                }
            }

            if let Some(flip_flop) = flip_flops.get_mut(id) {
                if id == "broadcaster" {
                    for target in flip_flop.1.iter() {
                        q.push_back((target, sig, id));
                    }
                } else {
                    // Flip-flop modules (prefix %) are either on or off; they are initially off. If a
                    // flip-flop module receives a high pulse, it is ignored and nothing happens. However,
                    // if a flip-flop module receives a low pulse, it flips between on and off. If it was
                    // off, it turns on and sends a high pulse. If it was on, it turns off and sends a low
                    // pulse.
                    if !sig {
                        flip_flop.0 ^= true;
                        for target in flip_flop.1.iter() {
                            q.push_back((target, flip_flop.0, id));
                        }
                    }
                }
            } else if let Some(conjunction) = conjunctions.get_mut(id) {
                // Conjunction modules (prefix &) remember the type of the most recent pulse
                // received from each of their connected input modules; they initially default to
                // remembering a low pulse for each input. When a pulse is received, the
                // conjunction module first updates its memory for that input. Then, if it
                // remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends
                // a high pulse.
                for input in conjunction.0.iter_mut() {
                    if input.0 == sender {
                        input.1 = sig;
                    }
                }
                let output = !conjunction.0.iter().all(|v| v.1);
                for target in conjunction.1.iter() {
                    q.push_back((target, output, id));
                }
            }
        }
    }

    cycle_lengths.into_iter().fold(1, |a, b| lcm(a, b))
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
fn test_part1_example_1() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    assert_eq!(part1(input), 32000000);
}

#[test]
fn test_part1_example_2() {
    let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
    assert_eq!(part1(input), 11687500);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 808146535);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 224602953547789);
}
