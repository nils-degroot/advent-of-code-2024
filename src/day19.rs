use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> impl ToString {
    fn decent(towels: &[&str], design: &str) -> bool {
        if design.is_empty() {
            return true;
        }

        for towel in towels {
            if design.starts_with(towel) && decent(towels, &design[towel.len()..]) {
                return true;
            }
        }

        false
    }

    let mut parts = input.split("\n\n");

    let towels = parts
        .next()
        .expect("Could not find towels")
        .split(", ")
        .collect::<Vec<_>>();

    parts
        .next()
        .expect("Could not find designs")
        .lines()
        .filter(|design| decent(&towels, design))
        .count()
}

pub fn part2(input: String) -> impl ToString {
    fn decent(
        towels: &HashSet<&str>,
        store: &mut HashMap<String, usize>,
        longest_towel: usize,
        design: &str,
    ) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(value) = store.get(design) {
            return *value;
        }

        let count = (1..=longest_towel.min(design.len()))
            .map(|idx| {
                if towels.contains(&design[..idx]) {
                    decent(towels, store, longest_towel, &design[idx..])
                } else {
                    0
                }
            })
            .sum();

        store.insert(design.to_string(), count);
        count
    }

    let mut parts = input.split("\n\n");

    let towels = parts
        .next()
        .expect("Could not find towels")
        .split(", ")
        .collect::<HashSet<_>>();

    let longest = towels
        .iter()
        .map(|t| t.len())
        .max()
        .expect("Could not find any towels");

    parts
        .next()
        .expect("Could not find designs")
        .lines()
        .map(|design| decent(&towels, &mut HashMap::new(), longest, design))
        .sum::<usize>()
}
