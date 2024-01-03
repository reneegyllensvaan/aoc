pub fn main(target: &[String], bench: bool) {
    let fns: Vec<(&'static str, fn(bool))> = vec![
        ("day01", crate::day01::main),
        ("day02", crate::day02::main),
        ("day03", crate::day03::main),
        ("day04", crate::day04::main),
        ("day05", crate::day05::main),
        ("day06", crate::day06::main),
        ("day07", crate::day07::main),
        ("day08", crate::day08::main),
        ("day09", crate::day09::main),
        ("day10", crate::day10::main),
        ("day11", crate::day11::main),
        ("day12", crate::day12::main),
        ("day13", crate::day13::main),
        ("day14", crate::day14::main),
        ("day15", crate::day15::main),
        ("day16", crate::day16::main),
        ("day17", crate::day17::main),
        ("day18", crate::day18::main),
        ("day19", crate::day19::main),
        ("day20", crate::day20::main),
        ("day21", crate::day21::main),
        ("day22", crate::day22::main),
        ("day23", crate::day23::main),
        ("day24", crate::day24::main),
        ("day25", crate::day25::main),
        // [NEXT DAY]
    ];
    for t in target {
        for (name, f) in &fns {
            if t == "all" || t == *name {
                println!("\n{name}:");
                f(bench);
            }
        }
    }
}
