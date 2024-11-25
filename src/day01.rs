use crate::hashmap;

pub fn part1(input: String) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let p = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();

            let head = p.first().expect("Could not find head");
            let tail = p.last().expect("Could not find tail");

            format!("{head}{tail}")
                .parse::<usize>()
                .expect("Could not parse piece")
        })
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    let lookup = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    input
        .lines()
        .map(|line| {
            let (lhs, _) = lookup
                .iter()
                .flat_map(|(text, digit)| {
                    [
                        line.find(&digit.to_string()).map(|pos| (digit, pos)),
                        line.find(text).map(|pos| (digit, pos)),
                    ]
                })
                .flatten()
                .min_by_key(|(_, p)| *p)
                .expect("Could not find head");

            let (rhs, _) = lookup
                .iter()
                .flat_map(|(text, digit)| {
                    [
                        line.rfind(&digit.to_string()).map(|pos| (digit, pos)),
                        line.rfind(text).map(|pos| (digit, pos)),
                    ]
                })
                .flatten()
                .max_by_key(|(_, p)| *p)
                .expect("Could not find tail");

            format!("{lhs}{rhs}")
                .parse::<usize>()
                .expect("Failed to parse values")
        })
        .sum::<usize>()
}
