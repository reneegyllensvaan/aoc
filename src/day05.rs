use std::{collections::HashMap, ops::RangeInclusive};

use itertools::Itertools;

fn part1(input: &str) -> i64 {
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
fn part2(input: &str) -> i64 {
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

fn range_intersect(
    a: &RangeInclusive<i64>,
    b: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    let start = *a.start().max(b.start());
    let end = *a.end().min(b.end());
    if start < end {
        Some(start..=end)
    } else {
        None
    }
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
fn part2_brute(input: &str) -> i64 {
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

    seeds
        .into_iter()
        .map(|seed_range| {
            let mut result = i64::MAX;
            for seed in seed_range {
                result = result.min(traverse_maps(seed, &maps));
            }
            result
        })
        .min()
        .unwrap()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day05").unwrap();

    // This one takes a couple minutes to run
    // println!("part2_brute(input): {:?}", part2_brute(&input));

    let iters = 1000;

    let fns: [(&'static str, fn(&str) -> i64); 2] = [("part1", part1), ("part2", part2)];

    for (name, f) in fns {
        println!("{name}: {}", f(&input));
    }

    for (name, f) in fns {
        let begin = std::time::Instant::now();
        for _ in 0..iters {
            f(&input);
        }
        let end = std::time::Instant::now();
        println!(
            "{} {} in: {}us ({}us/iter)",
            iters,
            name,
            (end - begin).as_micros(),
            (end - begin).as_micros() / iters
        );
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
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string("input/day05").unwrap();
    assert_eq!(part1(&input), 261668924);
}

#[test]
fn test_range_intersect() {
    assert_eq!(range_intersect(&(10..=50), &(5..=15)), Some(10..=15));
    assert_eq!(range_intersect(&(5..=50), &(10..=15)), Some(10..=15));
    assert_eq!(range_intersect(&(12..=20), &(10..=15)), Some(12..=15));
    assert_eq!(range_intersect(&(4..=5), &(10..=15)), None);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string("input/day05").unwrap();
    assert_eq!(part2(&input), 24261545);
}
