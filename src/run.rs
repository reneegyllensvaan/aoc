fn main() {
    let mut target = std::env::args().skip(1).collect::<Vec<_>>();
    if target.len() == 0 {
        target.push("all".to_string());
    }
    let fns: Vec<(&'static str, fn())> = vec![
        ("day01", aoc23::day01::main),
        ("day02", aoc23::day02::main),
        ("day03", aoc23::day03::main),
        ("day04", aoc23::day04::main),
        ("day05", aoc23::day05::main),
        ("day06", aoc23::day06::main),
        ("day07", aoc23::day07::main),
        ("day08", aoc23::day08::main),
        ("day09", aoc23::day09::main),
        ("day10", aoc23::day10::main),
        ("day11", aoc23::day11::main),
        ("day12", aoc23::day12::main),
        ("day13", aoc23::day13::main),
    ];
    for t in target {
        for (name, f) in &fns {
            if t == "all" || t == *name {
                println!("\n{name}:");
                f();
            }
        }
    }
}
