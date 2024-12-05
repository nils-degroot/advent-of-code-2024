use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> impl ToString {
    let mut split = input.split("\n\n");

    let ordering_rules = split
        .next()
        .expect("Could not find ordering rules")
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            (
                split
                    .next()
                    .expect("Could not find lhs")
                    .parse::<usize>()
                    .expect("Failed to parse number"),
                split
                    .next()
                    .expect("Could not find rhs")
                    .parse::<usize>()
                    .expect("Failed to parse number"),
            )
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, (k, v)| {
            if let Some(value) = acc.get_mut(&k) {
                value.push(v);
            } else {
                acc.insert(k, vec![v]);
            }
            acc
        });

    let mut sum = 0;

    'line: for line in split.next().expect("Could not find updates").lines() {
        let line = line
            .split(",")
            .map(|num| num.parse::<usize>().expect("Could not parse number"))
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();

        for num in &line {
            let before = ordering_rules
                .get(num)
                .cloned()
                .unwrap_or_default()
                .iter()
                .all(|num| !seen.contains(num));

            if !before {
                continue 'line;
            }

            seen.insert(*num);
        }

        sum += line[line.len() / 2];
    }

    sum
}

pub fn part2(input: String) -> impl ToString {
    let mut split = input.split("\n\n");

    let ordering_rules = split
        .next()
        .expect("Could not find ordering rules")
        .lines()
        .map(|line| {
            let mut split = line.split("|");
            (
                split
                    .next()
                    .expect("Could not find lhs")
                    .parse::<usize>()
                    .expect("Failed to parse number"),
                split
                    .next()
                    .expect("Could not find rhs")
                    .parse::<usize>()
                    .expect("Failed to parse number"),
            )
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, (k, v)| {
            if let Some(value) = acc.get_mut(&k) {
                value.push(v);
            } else {
                acc.insert(k, vec![v]);
            }
            acc
        });

    let mut sum = 0;

    for line in split.next().expect("Could not find updates").lines() {
        let mut line = line
            .split(",")
            .map(|num| num.parse::<usize>().expect("Could not parse number"))
            .collect::<Vec<_>>();

        let mut invalid = false;
        let mut idx = 0;

        while let Some(num) = line.get(idx) {
            let rule_broken = ordering_rules
                .get(num)
                .cloned()
                .unwrap_or_default()
                .iter()
                .any(|num| line[..idx].contains(num));

            if rule_broken {
                invalid = true;
                line.swap(idx, idx - 1);
                idx = 0;
            } else {
                idx += 1;
            }
        }

        if invalid {
            sum += line[line.len() / 2];
        }
    }

    sum
}
