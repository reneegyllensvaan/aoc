fn main() {
    let mut target = std::env::args().skip(1).collect::<Vec<_>>();
    if target.len() == 0 {
        target.push("all".to_string());
    }
    aoc23::run::main(&target, false);
}
