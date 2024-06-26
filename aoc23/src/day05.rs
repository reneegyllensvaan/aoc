use std::{cmp::Reverse, collections::HashMap, ops::RangeInclusive};
use utils::range_intersect;

use indicatif::{
    MultiProgress, ParallelProgressIterator, ProgressBar, ProgressDrawTarget, ProgressIterator,
    ProgressStyle,
};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part1(input: &str) -> i64 {
    let mut result: i64 = i64::MAX;

    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = HashMap::<(&str, &str), Vec<(i64, i64, i64)>>::new();
    let mut current_map: Option<((&str, &str), Vec<(i64, i64, i64)>)> = None;
    for line in lines {
        if line.len() == 0 {
            if let Some((path, vec)) = current_map.take() {
                maps.insert(path, vec);
            }
            continue;
        }

        if current_map.is_none() {
            let mapping = line.split_once(' ').unwrap().0.split('-').collect_vec();
            current_map = Some(((mapping[0], mapping[2]), Vec::new()));
        } else if let Some((_, ref mut v)) = &mut current_map {
            let mut mapping = line.split(' ').map(|v| v.parse::<i64>().unwrap());
            let to = mapping.next().unwrap();
            let from = mapping.next().unwrap();
            let len = mapping.next().unwrap();
            v.push((to, from, len));
        }
    }
    if let Some((path, vec)) = current_map.take() {
        maps.insert(path, vec);
    }

    for seed in seeds {
        result = result.min(traverse_maps(seed, &maps));
    }

    result
}
fn traverse_maps(seed: i64, maps: &HashMap<(&str, &str), Vec<(i64, i64, i64)>>) -> i64 {
    let soil = maps
        .get(&("seed", "soil"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if seed >= *from && seed <= (from + len - 1) {
                Some(to + seed - from)
            } else {
                None
            }
        })
        .unwrap_or(seed);

    let fertilizer = maps
        .get(&("soil", "fertilizer"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if soil >= *from && soil <= (from + len - 1) {
                Some(to + soil - from)
            } else {
                None
            }
        })
        .unwrap_or(soil);

    let water = maps
        .get(&("fertilizer", "water"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if fertilizer >= *from && fertilizer <= (from + len - 1) {
                Some(to + fertilizer - from)
            } else {
                None
            }
        })
        .unwrap_or(fertilizer);

    let light = maps
        .get(&("water", "light"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if water >= *from && water <= (from + len - 1) {
                Some(to + water - from)
            } else {
                None
            }
        })
        .unwrap_or(water);

    let temperature = maps
        .get(&("light", "temperature"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if light >= *from && light <= (from + len - 1) {
                Some(to + light - from)
            } else {
                None
            }
        })
        .unwrap_or(light);

    let humidity = maps
        .get(&("temperature", "humidity"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if temperature >= *from && temperature <= (from + len - 1) {
                Some(to + temperature - from)
            } else {
                None
            }
        })
        .unwrap_or(temperature);

    let location = maps
        .get(&("humidity", "location"))
        .unwrap()
        .iter()
        .find_map(|(to, from, len)| {
            if humidity >= *from && humidity <= (from + len - 1) {
                Some(to + humidity - from)
            } else {
                None
            }
        })
        .unwrap_or(humidity);

    location
}

/// Version 1.
///
/// This one was fun. I might have overcomplicated it a bit, but this thing feels pretty solid
/// still.
pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .filter_map(|mut v| {
            let start = v.next()?;
            let num = v.next()?;
            Some(start..=(start + num - 1))
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    let mut maps = HashMap::<(&str, &str), Vec<(i64, RangeInclusive<i64>)>>::new();
    let mut current_map: Option<((&str, &str), Vec<(i64, RangeInclusive<i64>)>)> = None;
    for line in lines {
        if line.len() == 0 {
            if let Some((path, vec)) = current_map.take() {
                maps.insert(path, vec);
            }
            continue;
        }

        if current_map.is_none() {
            let mapping = line.split_once(' ').unwrap().0.split('-').collect_vec();
            current_map = Some(((mapping[0], mapping[2]), Vec::new()));
        } else if let Some((_, ref mut v)) = &mut current_map {
            let mut mapping = line.split(' ').map(|v| v.parse::<i64>().unwrap());
            let to = mapping.next().unwrap();
            let from = mapping.next().unwrap();
            let len = mapping.next().unwrap();
            v.push((to - from, from..=(from + len)));
        }
    }
    if let Some((path, vec)) = current_map.take() {
        maps.insert(path, vec);
    }

    let soil = map_ranges(&seeds, maps.get(&("seed", "soil")).unwrap());
    let fertilizer = map_ranges(&soil, maps.get(&("soil", "fertilizer")).unwrap());
    let water = map_ranges(&fertilizer, maps.get(&("fertilizer", "water")).unwrap());
    let light = map_ranges(&water, maps.get(&("water", "light")).unwrap());
    let temperature = map_ranges(&light, maps.get(&("light", "temperature")).unwrap());
    let humidity = map_ranges(
        &temperature,
        maps.get(&("temperature", "humidity")).unwrap(),
    );
    let location = map_ranges(&humidity, maps.get(&("humidity", "location")).unwrap());

    location.into_iter().map(|v| *v.start()).min().unwrap()
}

fn map_ranges(
    ranges: &[RangeInclusive<i64>],
    map: &[(i64, RangeInclusive<i64>)],
) -> Vec<RangeInclusive<i64>> {
    ranges
        .iter()
        .map(|r| map_range(r.clone(), map))
        .flatten()
        .collect()
}
fn map_range(
    mut range: RangeInclusive<i64>,
    map: &[(i64, RangeInclusive<i64>)],
) -> Vec<RangeInclusive<i64>> {
    let mut result = Vec::<RangeInclusive<i64>>::new();

    // First, any out-of-bounds regions get mapped to themselves
    let min = *map.iter().map(|v| v.1.start()).min().unwrap();
    let max = *map.iter().map(|v| v.1.end()).max().unwrap();
    if *range.start() < min {
        result.push(*range.start()..=(min - 1));
        range = min..=*range.end();
    }
    if *range.end() > max {
        result.push((max + 1).max(*range.start())..=*range.end());
        range = *range.start()..=max;
    }

    // Then, any in-bounds regions get mapped with an offset
    let mut range_before = range.clone();
    while range.end() > range.start() {
        for (offset, map_region) in map {
            let Some(source_range) = range_intersect(&map_region, &range) else {
                continue;
            };

            if source_range.start() != range.start() {
                continue;
            }
            range = *source_range.end()..=*range.end();

            let target_range = (source_range.start() + offset)..=(source_range.end() + offset);
            result.push(target_range);
        }

        if range.end() == range.start() {
            break;
        }

        // If we've gone through all the regions and not found a head section, we're in an
        // inter-region gap in the map. Find how long the gap is, and push an identity-mapped
        // section for that slice.
        let Some(min) = map
            .iter()
            .filter(|v| *v.1.start() >= *range.start())
            .map(|v| *v.1.start())
            .min()
        else {
            panic!("missing head: {range:?} -> {map:?}");
        };
        result.push(*range.start()..=(min - 1));
        range = min..=*range.end();

        if range == range_before {
            panic!("stuck on range: {range:?}");
        }
        range_before = range.clone();
    }

    result
}

/// Version 2.
///
/// A solution to part2 that solves it by using the part1 solution on one thread per seed range
/// (incidentally, i have 10 cores and my input has 10 seed ranges).
///
/// Parallel version took about 160 seconds on my machine - i could have just waited! (hell, isn't
/// that 80ns/iter? goddamn nanoseconds?)
///
/// Single-threaded version took 635 seconds. That's still very good lol - 0.31us per seed. I guess
/// parsing was the majority of time in part1.
#[allow(dead_code)]
pub fn part2_brute(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .chunks(2)
        .into_iter()
        .filter_map(|mut v| {
            let start = v.next()?;
            let num = v.next()?;
            Some(start..=(start + num - 1))
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    let mut maps = HashMap::<(&str, &str), Vec<(i64, i64, i64)>>::new();
    let mut current_map: Option<((&str, &str), Vec<(i64, i64, i64)>)> = None;
    for line in lines {
        if line.len() == 0 {
            if let Some((path, vec)) = current_map.take() {
                maps.insert(path, vec);
            }
            continue;
        }

        if current_map.is_none() {
            let mapping = line.split_once(' ').unwrap().0.split('-').collect_vec();
            current_map = Some(((mapping[0], mapping[2]), Vec::new()));
        } else if let Some((_, ref mut v)) = &mut current_map {
            let mut mapping = line.split(' ').map(|v| v.parse::<i64>().unwrap());
            let to = mapping.next().unwrap();
            let from = mapping.next().unwrap();
            let len = mapping.next().unwrap();
            v.push((to, from, len));
        }
    }
    if let Some((path, vec)) = current_map.take() {
        maps.insert(path, vec);
    }

    let progress = MultiProgress::new();
    let main_bar = ProgressBar::new(seeds.len() as u64).with_style(
        ProgressStyle::with_template(
            "Brute-forcing seed ranges: {bar:40.red/yellow} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    if cfg!(test) {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    progress.add(main_bar.clone());
    main_bar.tick();
    seeds
        .into_par_iter()
        .map(|seed_range| {
            let mut result = i64::MAX;
            let (a, b) = seed_range.clone().into_inner();
            let bar_style = ProgressStyle::with_template(&format!(
                "       start={a:<12}: {}",
                "{bar:40.cyan/blue} {pos:>13}/{len:13} {msg}"
            ))
            .unwrap();
            let bar = ProgressBar::new((b - a) as u64).with_style(bar_style);
            progress.add(bar.clone());

            for seed in seed_range.progress_with(bar) {
                result = result.min(traverse_maps(seed, &maps));
            }
            result
        })
        .progress_with(main_bar)
        .min()
        .unwrap()
}

/// Version 3.
///
/// A faster brute-force solution to part2.
///
/// Previous version was doing a hash and hashmap lookup for every seed. Here, i just get the
/// entries from the hash map once and use them as variables.
///
/// I also sort the maps in descending order of _size_. Figured larger maps are more likely to
/// contain our value, so we can break early more often.
///
/// This one ran in 14 seconds on my machine. imo that's very respectable.
pub fn part2_brute_faster(input: &str) -> i64 {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .chunks(2)
        .into_iter()
        .filter_map(|mut v| {
            let start = v.next()?;
            let num = v.next()?;
            Some(start..=(start + num - 1))
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    let mut maps = HashMap::<(&str, &str), Vec<(i64, i64, i64)>>::new();
    let mut current_map: Option<((&str, &str), Vec<(i64, i64, i64)>)> = None;
    for line in lines {
        if line.len() == 0 {
            if let Some((path, vec)) = current_map.take() {
                maps.insert(path, vec);
            }
            continue;
        }

        if current_map.is_none() {
            let mapping = line.split_once(' ').unwrap().0.split('-').collect_vec();
            current_map = Some(((mapping[0], mapping[2]), Vec::new()));
        } else if let Some((_, ref mut v)) = &mut current_map {
            let mut mapping = line.split(' ').map(|v| v.parse::<i64>().unwrap());
            let to = mapping.next().unwrap();
            let from = mapping.next().unwrap();
            let len = mapping.next().unwrap();
            v.push((to, from, len));
        }
    }
    if let Some((path, vec)) = current_map.take() {
        maps.insert(path, vec);
    }

    for (_, map) in maps.iter_mut() {
        map.sort_unstable_by_key(|v| -v.2);
    }

    let seed_to_soil = maps.get(&("seed", "soil")).unwrap();
    let soil_to_fertilizer = maps.get(&("soil", "fertilizer")).unwrap();
    let fertilizer_to_water = maps.get(&("fertilizer", "water")).unwrap();
    let water_to_light = maps.get(&("water", "light")).unwrap();
    let light_to_temperature = maps.get(&("light", "temperature")).unwrap();
    let temperature_to_humidity = maps.get(&("temperature", "humidity")).unwrap();
    let humidity_to_location = maps.get(&("humidity", "location")).unwrap();
    fn map_through(id: i64, maps: &[(i64, i64, i64)]) -> i64 {
        maps.iter()
            .find_map(|(to, from, len)| {
                if id >= *from && id <= (from + len - 1) {
                    Some(to + id - from)
                } else {
                    None
                }
            })
            .unwrap_or(id)
    }
    seeds
        .into_par_iter()
        .map(|seed_range| {
            let mut result = i64::MAX;
            for seed in seed_range {
                let soil = map_through(seed, &seed_to_soil);
                let fertilizer = map_through(soil, &soil_to_fertilizer);
                let water = map_through(fertilizer, &fertilizer_to_water);
                let light = map_through(water, &water_to_light);
                let temperature = map_through(light, &light_to_temperature);
                let humidity = map_through(temperature, &temperature_to_humidity);
                let location = map_through(humidity, &humidity_to_location);

                result = result.min(location);
            }
            result
        })
        .min()
        .unwrap()
}

/// Version 4.
///
/// This one is slower, but uses the [`indicatif`] crate to show progress bars for each seed range.
///
/// It takes like twice as long as without progress bars because the loop is so trivial, but it
/// looks cool so i keep it in here.
pub fn part2_brute_faster_with_progress(input: &str) -> i64 {
    let (seeds, mut maps) = parse_part2(input);

    for map in maps.iter_mut() {
        map.sort_unstable_by_key(|v| -v.2);
    }

    let progress = MultiProgress::new();
    let main_bar = ProgressBar::new(seeds.len() as u64).with_style(
        ProgressStyle::with_template(
            "Brute-forcing seed ranges: {bar:40.red/yellow} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    if cfg!(test) {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }
    progress.add(main_bar.clone());
    main_bar.tick();

    fn map_through(seed: i64, (to, from, len): &(i64, i64, i64)) -> Option<i64> {
        if seed >= *from && seed <= (from + len - 1) {
            Some(to + seed - from)
        } else {
            None
        }
    }
    fn map_through_all(id: i64, maps: &[(i64, i64, i64)]) -> i64 {
        maps.iter()
            .find_map(|map| map_through(id, map))
            .unwrap_or(id)
    }
    seeds
        .into_par_iter()
        .map(|seed_range| {
            let mut result = i64::MAX;
            let (a, b) = seed_range.clone().into_inner();
            let bar_style = ProgressStyle::with_template(&format!(
                "       start={a:<12}: {}",
                "{bar:40.cyan/blue} {pos:>13}/{len:13} {msg}"
            ))
            .unwrap();
            let bar = ProgressBar::new((b - a) as u64).with_style(bar_style);
            progress.add(bar.clone());

            for mut seed in seed_range.progress_with(bar) {
                for map in maps.iter() {
                    seed = map_through_all(seed, &map);
                }
                result = result.min(seed);
            }
            result
        })
        .progress_with(main_bar)
        .min()
        .unwrap()
}

/// Version 5.
///
/// So by changing the loop order, I can make the brute force faster yet, by increasing
/// utilization. One task per range meant the shortest range would finish quickly and then we'd be
/// bounded by the longest range.
///
/// Now, we instead fork out NUM_CPUS tasks per seed range. Turns out that's way faster.
///
/// This one runs in about 7s on my machine.
pub fn part2_brute_faster_2(input: &str) -> i64 {
    let (seeds, mut maps) = parse_part2(input);

    for map in maps.iter_mut() {
        map.sort_unstable_by_key(|v| Reverse(v.1 - v.0));
    }

    let map_through = |mut id: i64| -> i64 {
        for map in maps.iter() {
            if let Some(offset) = map
                .iter()
                .filter_map(|&(a, b, o)| if id >= a && id <= b { Some(o) } else { None })
                .next()
            {
                id += offset;
            }
        }
        id
    };

    seeds
        .into_iter()
        .map(|s| s.into_par_iter().map(map_through).min().unwrap())
        .min()
        .unwrap()
}

/// Version 6.
///
/// We can brute force even faster. This one relies on having a clone of the maps for each thread,
/// so that each thread can mutate it.
///
/// The insight here is that the same map entry will likely occur consecutively. Thus, the
/// optimization I've done is that each time a map entry is matched, that map entry is bubbled
/// backwards towards the start of the map vector. That way, consecutively accessed map entries are
/// faster, which speeds up the common case quite a bit.
///
/// This one runs in about 5s on my machine.
pub fn part2_brute_faster_3(input: &str) -> i64 {
    let (seeds, mut maps) = parse_part2(input);

    for map in maps.iter_mut() {
        map.sort_unstable_by_key(|v| Reverse(v.1 - v.0));
    }

    let mut result = i64::MAX;
    for seed_range in seeds {
        let pr = std::thread::scope(|scope| {
            let num_cores = std::thread::available_parallelism().unwrap().get() as i64;
            let mut threads = vec![];
            let start = *seed_range.start();
            let end = *seed_range.end();
            let chunk_size = ((end - start) / num_cores) + 2;

            for cix in 0..num_cores {
                let a = start + chunk_size * cix;
                let b = end.min(start + chunk_size * (cix + 1));
                // let mut founds = vec![0usize; 64];
                let mut maps = maps.clone();
                threads.push(scope.spawn(move || {
                    let mut result = i64::MAX;
                    for mut id in a..=b {
                        for map in maps.iter_mut() {
                            let mut found: Option<usize> = None;
                            for (ix, &(a, b, o)) in map.iter().enumerate() {
                                if id >= a && id <= b {
                                    id += o;
                                    // founds[ix] += 1;
                                    if ix > 0 {
                                        found = Some(ix);
                                    }
                                    break;
                                }
                            }
                            if let Some(ix) = found {
                                map.swap(ix, ix - 1);
                            }
                        }
                        result = result.min(id);
                    }
                    // println!("founds ({cix}): {:?}", &founds[0..16]);
                    result
                }));
            }
            threads.into_iter().map(|t| t.join().unwrap()).min()
        });
        result = result.min(pr.unwrap());
    }
    result
}

fn parse_part2(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<Vec<(i64, i64, i64)>>) {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .chunks(2)
        .into_iter()
        .filter_map(|mut v| {
            let start = v.next()?;
            let num = v.next()?;
            Some(start..=(start + num - 1))
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    let maps = lines
        .filter(|l| !l.is_empty())
        .group_by(|l| l.starts_with(char::is_alphabetic))
        .into_iter()
        .filter(|v| !v.0)
        .map(|ls| {
            ls.1.map(|line| {
                let mut m = line.split(' ').map(|v| v.parse::<i64>().unwrap());
                let (to, from, len) = (m.next().unwrap(), m.next().unwrap(), m.next().unwrap());
                (from, from + len - 1, to - from)
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string("input/day05").unwrap();

    // // This one takes a couple minutes to run
    let begin = std::time::Instant::now();
    print!(
        "part2_brute_faster_3(input): {:?}",
        part2_brute_faster_3(&input)
    );
    let end = std::time::Instant::now();
    println!(" in {}ms", (end - begin).as_millis());

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 2] = [("part1", part1), ("part2", part2)];

    for (name, f) in fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
        for (name, f) in fns {
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
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(part1(input), 35);
}

#[test]
fn test_part2_example() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
    assert_eq!(part2(input), 46);
    assert_eq!(part2_brute(input), 46);
    assert_eq!(part2_brute_faster(input), 46);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day05").unwrap();
    assert_eq!(part1(&input), 261668924);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day05").unwrap();
    assert_eq!(part2(&input), 24261545);
}
