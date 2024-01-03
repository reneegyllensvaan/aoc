use z3::ast::Ast;

static INPUT_FILE: &str = "input/day24";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

type V2 = (f64, f64);
type L2 = (V2, V2);

pub fn part1_example(input: &str) -> i64 {
    part1(input, 7, 27)
}
pub fn part1_real(input: &str) -> i64 {
    part1(input, 200000000000000, 400000000000000)
}
pub fn part1(input: &str, min: i64, max: i64) -> i64 {
    let stones = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let mut p = p.split(", ").map(|v| v.trim().parse::<f64>().unwrap());
            let mut v = v.split(", ").map(|v| v.trim().parse::<f64>().unwrap());
            (
                (p.next().unwrap(), p.next().unwrap()),
                (v.next().unwrap(), v.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for (ix, a) in stones.iter().enumerate() {
        for b in &stones[(ix + 1)..] {
            // println!("(a, b): {:?}", (a, b));
            let Some((x, y)) = intersection(*a, *b) else {
                continue;
            };
            let min = min as f64;
            let max = max as f64;
            let within = min <= x && x <= max && min <= y && y <= max;
            // println!("  (x, y): {:?}", (x, y));
            if within {
                // println!("  matching");
                result += 1;
            } else {
                // println!("  not matching");
            }
        }
    }

    result
}

fn slope_intercept(((x, y), (dx, dy)): L2) -> (f64, f64) {
    // y = m*x + b -> m = (y2-y1)/(x2-x1)
    let m = ((y + dy) - y) / ((x + dx) - x);
    // b = y - m*x
    let b = y - m * x;
    (m, b)
}

fn intersection(a: L2, b: L2) -> Option<V2> {
    // Find how long it takes to intersect in general. I suppose I'm assuming that the
    // paths do in fact intersect?
    let (ma, ba) = slope_intercept(a);
    let (mb, bb) = slope_intercept(b);

    // lines are either parallel or within each other
    if ma == mb {
        return None;
    }

    // intersection point:
    // ma*x + ba = mb*x + bb
    // ma*x - mb*x = bb - ba
    // x(ma - mb) = bb - ba
    // x = (bb - ba) / (ma - mb)
    let x0 = (bb - ba) / (ma - mb);
    // plug x into either equation
    let y0 = ma * x0 + ba;

    let ((xa, _), (dxa, _)) = a;
    // make sure intersection is in future
    if dxa > 0. {
        // dxa is positive, so if x has decreased, then we've moved backwards in time
        if x0 < xa {
            return None;
        }
    } else if dxa < 0. {
        // dxa is negative, so if x has increased, then we've moved backwards in time
        if x0 > xa {
            return None;
        }
    }

    let ((xb, _), (dxb, _)) = b;
    // ditto for xb
    if dxb > 0. {
        if x0 < xb {
            return None;
        }
    } else if dxb < 0. {
        if x0 > xb {
            return None;
        }
    }

    Some((x0, y0))
}

pub fn part2(input: &str) -> i64 {
    // TODO: look into solving this without Z3
    use z3::ast;
    let stones = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let mut p = p.split(", ").map(|v| v.trim().parse::<i64>().unwrap());
            let mut v = v.split(", ").map(|v| v.trim().parse::<i64>().unwrap());
            (
                (p.next().unwrap(), p.next().unwrap(), p.next().unwrap()),
                (v.next().unwrap(), v.next().unwrap(), v.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);
    let x0 = ast::Int::new_const(&ctx, "x0");
    let y0 = ast::Int::new_const(&ctx, "y0");
    let z0 = ast::Int::new_const(&ctx, "z0");

    let dx0 = ast::Int::new_const(&ctx, "dx0");
    let dy0 = ast::Int::new_const(&ctx, "dy0");
    let dz0 = ast::Int::new_const(&ctx, "dz0");

    let solver = z3::Solver::new(&ctx);
    for (ix, ((x, y, z), (dx, dy, dz))) in stones.iter().enumerate() {
        let t = ast::Int::new_const(&ctx, ix as u32);
        solver.assert(&t.gt(&ast::Int::from_i64(&ctx, 0)));
        for (v0, dv0, v, dv) in [(&x0, &dx0, x, dx), (&y0, &dy0, y, dy), (&z0, &dz0, z, dz)] {
            let l = v0 + (dv0 * &t);
            let r = ast::Int::from_i64(&ctx, *v) + &t * ast::Int::from_i64(&ctx, *dv);
            solver.assert(&l._eq(&r));
        }
    }
    // println!("solver:\n{}", solver.to_string());

    solver.check();
    // println!("solver.check_sat(): {:?}", solver.check());
    let model = solver.get_model().unwrap();
    // println!("model: {:?}", model);

    // (assert (= (+ x0 (* dx t1)) (+ 19 (* -2 t1))))
    // (assert (= (+ y0 (* dy t1)) (+ 13 (* 1 t1))))
    // (assert (= (+ z0 (* dz t1)) (+ 30 (* -2 t1))))

    // (assert (= (+ x0 (* dx t2)) (+ 18 (* -1 t2))))
    // (assert (= (+ y0 (* dy t2)) (+ 19 (* -1 t2))))
    // (assert (= (+ z0 (* dz t2)) (+ 22 (* -2 t2))))

    // (assert (= (+ x0 (* dx t3)) (+ 20 (* -2 t3))))
    // (assert (= (+ y0 (* dy t3)) (+ 25 (* -2 t3))))
    // (assert (= (+ z0 (* dz t3)) (+ 34 (* -4 t3))))

    // (assert (= (+ x0 (* dx0 k!0)) (+ 19 (* k!0 (- 2)))))
    // (assert (= (+ y0 (* dy0 k!0)) (+ 13 (* k!0 1))))
    // (assert (= (+ z0 (* dz0 k!0)) (+ 30 (* k!0 (- 2)))))
    // (assert (= (+ x0 (* dx0 k!1)) (+ 18 (* k!1 (- 1)))))
    // (assert (= (+ y0 (* dy0 k!1)) (+ 19 (* k!1 (- 1)))))
    // (assert (= (+ z0 (* dz0 k!1)) (+ 22 (* k!1 (- 2)))))
    // (assert (= (+ x0 (* dx0 k!2)) (+ 20 (* k!2 (- 2)))))
    // (assert (= (+ y0 (* dy0 k!2)) (+ 25 (* k!2 (- 2)))))
    // (assert (= (+ z0 (* dz0 k!2)) (+ 34 (* k!2 (- 4)))))

    let x0 = model.eval(&x0, true).unwrap().as_i64().unwrap();
    let y0 = model.eval(&y0, true).unwrap().as_i64().unwrap();
    let z0 = model.eval(&z0, true).unwrap().as_i64().unwrap();
    // println!("(x0, y0, z0): {:?}", (x0, y0, z0));
    x0 + y0 + z0
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 1;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![("part1", part1_real), ("part2", part2)];

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
fn test_part1_example() {
    assert_eq!(part1_example(EXAMPLE_INPUT), 2);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 47);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1_real(&input), 0);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 0);
}

#[test]
fn test_intersection() {
    assert_eq!(
        intersection(((19.0, 13.0), (-2.0, 1.0)), ((18.0, 19.0), (-1.0, -1.0))),
        Some((14.333333333333334, 15.333333333333332))
    );

    assert_eq!(
        intersection(((19.0, 13.0), (-2.0, 1.0)), ((20.0, 25.0), (-2.0, -2.0))),
        Some((11.666666666666666, 16.666666666666668))
    );

    assert_eq!(
        intersection(((19.0, 13.0), (-2.0, 1.0)), ((12.0, 31.0), (-1.0, -2.0))),
        Some((6.2, 19.4))
    );
    assert_eq!(
        intersection(((19.0, 13.0), (-2.0, 1.0)), ((20.0, 19.0), (1.0, -5.0))),
        None
    );
    assert_eq!(
        intersection(((18.0, 19.0), (-1.0, -1.0)), ((20.0, 25.0), (-2.0, -2.0))),
        None
    );
    assert_eq!(
        intersection(((18.0, 19.0), (-1.0, -1.0)), ((12.0, 31.0), (-1.0, -2.0))),
        Some((-6.0, -5.0))
    );
    assert_eq!(
        intersection(((18.0, 19.0), (-1.0, -1.0)), ((20.0, 19.0), (1.0, -5.0))),
        None
    );
    assert_eq!(
        intersection(((20.0, 25.0), (-2.0, -2.0)), ((12.0, 31.0), (-1.0, -2.0))),
        Some((-2.0, 3.0))
    );
    assert_eq!(
        intersection(((20.0, 25.0), (-2.0, -2.0)), ((20.0, 19.0), (1.0, -5.0))),
        None
    );
    assert_eq!(
        intersection(((12.0, 31.0), (-1.0, -2.0)), ((20.0, 19.0), (1.0, -5.0))),
        None
    );
}
