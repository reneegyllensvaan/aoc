use crate::utils::range_intersect;
use std::{collections::HashMap, ops::RangeInclusive};

static INPUT_FILE: &str = "input/day19";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

pub fn part1(input: &str) -> i64 {
    #[derive(Debug)]
    enum Instr<'a> {
        Goto(&'a str),
        Accept,
        Reject,
        CmpBranch {
            component: &'a str,
            gt: bool,
            num: i64,
            then: Box<Instr<'a>>,
        },
    }

    let mut rules = HashMap::<&str, Vec<Instr<'_>>>::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (id, tail) = line.split_once('{').unwrap();
        let rule = rules.entry(id).or_default();
        for instr in (&tail[0..tail.len() - 1]).split(',') {
            if let Some((expr, then)) = instr.split_once(':') {
                rule.push(Instr::CmpBranch {
                    component: &expr[0..1],
                    gt: expr.chars().nth(1).unwrap() == '>',
                    num: (&expr[2..]).parse().unwrap(),
                    then: Box::new(match then {
                        "A" => Instr::Accept,
                        "R" => Instr::Reject,
                        v => Instr::Goto(v),
                    }),
                });
            } else {
                rule.push(match instr {
                    "A" => Instr::Accept,
                    "R" => Instr::Reject,
                    v => Instr::Goto(v),
                });
            }
        }
    }

    let mut parts = Vec::<HashMap<&str, i64>>::new();
    for line in &mut lines {
        let mut part = HashMap::new();
        for component in (&line[1..line.len() - 1]).split(',') {
            let (p, num) = component.split_once('=').unwrap();
            part.insert(p, num.parse().unwrap());
        }
        parts.push(part);
    }

    let mut result = 0;
    for part in parts {
        let mut rule = rules.get("in").unwrap();
        'part: loop {
            for mut instr in rule {
                if let Instr::CmpBranch {
                    component,
                    gt,
                    num,
                    then,
                } = instr
                {
                    let component = part.get(component).unwrap();
                    let matches = if *gt {
                        component > num
                    } else {
                        component < num
                    };
                    if matches {
                        instr = then;
                    } else {
                        continue;
                    }
                };
                match instr {
                    Instr::Accept => {
                        result += part.values().sum::<i64>();
                        break 'part;
                    }
                    Instr::Reject => {
                        break 'part;
                    }
                    Instr::Goto(t) => {
                        rule = rules.get(t).unwrap();
                        continue 'part;
                    }
                    Instr::CmpBranch { .. } => unreachable!(),
                }
            }
        }
    }

    result
}

pub fn part2<'a>(input: &'a str) -> i64 {
    #[derive(Debug, Clone)]
    enum Instr {
        Goto(String),
        Accept,
        Reject,
        CmpBranch {
            component: char,
            gt: bool,
            num: i64,
            then: Box<Instr>,
        },
    }

    let mut rules = HashMap::<&str, Vec<Instr>>::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (id, tail) = line.split_once('{').unwrap();
        let rule = rules.entry(id).or_default();
        for instr in (&tail[0..tail.len() - 1]).split(',') {
            if let Some((expr, then)) = instr.split_once(':') {
                rule.push(Instr::CmpBranch {
                    component: expr.chars().next().unwrap(),
                    gt: expr.chars().nth(1).unwrap() == '>',
                    num: (&expr[2..]).parse().unwrap(),
                    then: Box::new(match then {
                        "A" => Instr::Accept,
                        "R" => Instr::Reject,
                        v => Instr::Goto(v.to_string()),
                    }),
                });
            } else {
                rule.push(match instr {
                    "A" => Instr::Accept,
                    "R" => Instr::Reject,
                    v => Instr::Goto(v.to_string()),
                });
            }
        }
    }

    type PartRange = HashMap<char, RangeInclusive<i64>>;
    let part = PartRange::from([
        ('x', 1..=4000),
        ('m', 1..=4000),
        ('a', 1..=4000),
        ('s', 1..=4000),
    ]);

    fn evaluate(part: PartRange, rule: &[Instr], rules: &HashMap<&str, Vec<Instr>>) -> i64 {
        match rule.first().unwrap() {
            Instr::Accept => part
                .values()
                .map(|r| r.end() - r.start() + 1)
                .product::<i64>(),
            Instr::Reject => {
                return 0;
            }
            Instr::Goto(t) => {
                let rule = rules.get(t.as_str()).unwrap();
                return evaluate(part, rule, rules);
            }
            Instr::CmpBranch {
                component,
                gt,
                num,
                then,
            } => {
                let cv = part.get(component).unwrap();
                let (matching, non_matching) = if *gt {
                    (
                        range_intersect(cv, &((num + 1)..=i64::MAX)),
                        range_intersect(cv, &(i64::MIN..=(*num))),
                    )
                } else {
                    (
                        range_intersect(cv, &(i64::MIN..=(num - 1))),
                        range_intersect(cv, &(*num..=i64::MAX)),
                    )
                };
                let mut result = 0;
                if let Some(r) = matching {
                    let mut new_part = part.clone();
                    new_part.insert(*component, r);
                    result += evaluate(new_part, &vec![then.as_ref().clone()], rules);
                }
                if let Some(r) = non_matching {
                    let mut new_part = part.clone();
                    new_part.insert(*component, r);
                    result += evaluate(new_part, &rule[1..], rules);
                }
                result
            }
        }
    }

    evaluate(part, rules.get("in").unwrap(), &rules)
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
    assert_eq!(part1(EXAMPLE_INPUT), 19114);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 167409079868000);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 480738);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 131550418841958);
}
