fn main() {
    let mut target = std::env::args().skip(1).collect::<Vec<_>>();
    if target.len() == 0 {
        target.push("all".to_string());
    }
    let fns: Vec<(&'static str, fn(bool))> = vec![
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
        ("day14", aoc23::day14::main),
        ("day15", aoc23::day15::main),
        ("day16", aoc23::day16::main),
        ("day17", aoc23::day17::main),
        ("day18", aoc23::day18::main),
        ("day19", aoc23::day19::main),
        ("day20", aoc23::day20::main),
        ("day21", aoc23::day21::main),
        ("day22", aoc23::day22::main),
        ("day23", aoc23::day23::main),
        ("day24", aoc23::day24::main),
        ("day25", aoc23::day25::main),
        // [NEXT DAY]
    ];
    for t in target {
        for (name, f) in &fns {
            if t == "all" || t == *name {
                println!("\n{name}:");
                f(false);
            }
        }
    }
}
