use std::collections::HashMap;

pub fn part1(input: String) -> impl ToString {
    let mut numbers = input
        .split_whitespace()
        .map(|num| num.parse::<usize>().expect("Could not parse number"))
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut new = vec![];

        for number in numbers {
            match number {
                0 => {
                    new.push(1);
                }
                number if number.to_string().len() % 2 == 0 => {
                    let s = number.to_string();

                    let lhs = &s[..(s.len() / 2)];
                    new.push(lhs.parse().expect("Could not parse number"));

                    let rhs = &s[(s.len() / 2)..];
                    new.push(rhs.parse().expect("Could not parse number"));
                }
                number => new.push(number * 2024),
            }
        }

        numbers = new;
    }

    numbers.len()
}

pub fn part2(input: String) -> impl ToString {
    let mut numbers = input
        .split_whitespace()
        .map(|num| num.parse::<usize>().expect("Could not parse number"))
        .fold(HashMap::<usize, usize>::new(), |mut acc, v| {
            if let Some(n) = acc.get_mut(&v) {
                *n += 1;
            } else {
                acc.insert(v, 1);
            }
            acc
        });

    for _ in 0..75 {
        let mut new = HashMap::new();

        for (number, count) in numbers {
            match number {
                0 => {
                    if let Some(n) = new.get_mut(&1) {
                        *n += count
                    } else {
                        new.insert(1, count);
                    }
                }
                number if number.to_string().len() % 2 == 0 => {
                    let s = number.to_string();

                    let lhs = s[..(s.len() / 2)].parse().expect("Could not parse number");
                    if let Some(n) = new.get_mut(&lhs) {
                        *n += count
                    } else {
                        new.insert(lhs, count);
                    }

                    let rhs = s[(s.len() / 2)..].parse().expect("Could not parse number");
                    if let Some(n) = new.get_mut(&rhs) {
                        *n += count
                    } else {
                        new.insert(rhs, count);
                    }
                }
                number => {
                    if let Some(n) = new.get_mut(&(number * 2024)) {
                        *n += count
                    } else {
                        new.insert(number * 2024, count);
                    }
                }
            }
        }

        numbers = new;
    }

    numbers.values().sum::<usize>()
}
