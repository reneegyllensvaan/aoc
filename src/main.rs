static USAGE: &str = "usage: aoc <22|23> [target...]";

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(year) = args.next() else {
        eprintln!("{}", USAGE);
        return;
    };
    let fns: Vec<(&'static str, fn(&[String], bool))> =
        vec![("23", aoc23::run::main), ("22", aoc22::run::main)];
    let Some((name, f)) = fns.into_iter().find(|(n, _)| *n == year) else {
        eprintln!("unknown year: {}", year);
        return;
    };

    let mut target = args.collect::<Vec<_>>();
    if target.is_empty() {
        target.push("all".to_string());
    }
    let mut p = std::env::current_dir().unwrap();
    p.push(format!("aoc{name}"));
    std::env::set_current_dir(p).ok();
    f(&target, false);
    eprintln!("{}", USAGE);
}
