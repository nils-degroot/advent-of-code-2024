pub fn part1(input: String) -> impl ToString {
    fn sums_up(desired: usize, acc: usize, numbers: &[usize]) -> bool {
        if numbers.is_empty() {
            return desired == acc;
        }

        let num = numbers[0];
        let other = &numbers[1..];

        if num != 0 && sums_up(desired, acc * num, other) {
            return true;
        }
        if sums_up(desired, acc + num, other) {
            return true;
        }

        false
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            (
                parts
                    .next()
                    .expect("Could not find head")
                    .parse()
                    .expect("Invalid test value"),
                parts
                    .next()
                    .expect("Could not find tail")
                    .split_whitespace()
                    .map(|number| number.parse::<usize>().expect("Invalid number passed"))
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(test_value, rest)| sums_up(*test_value, 0, rest))
        .map(|(test_value, _)| test_value)
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    fn sums_up(desired: usize, acc: usize, numbers: &[usize]) -> bool {
        if numbers.is_empty() {
            return desired == acc;
        }

        let num = numbers[0];
        let other = &numbers[1..];

        if num != 0 && sums_up(desired, acc * num, other) {
            return true;
        }
        if sums_up(desired, acc + num, other) {
            return true;
        }
        if sums_up(
            desired,
            format!("{acc}{num}")
                .parse()
                .expect("Could not parse contcat"),
            other,
        ) {
            return true;
        }

        false
    }

    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            (
                parts
                    .next()
                    .expect("Could not find head")
                    .parse()
                    .expect("Invalid test value"),
                parts
                    .next()
                    .expect("Could not find tail")
                    .split_whitespace()
                    .map(|number| number.parse::<usize>().expect("Invalid number passed"))
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(test_value, rest)| sums_up(*test_value, 0, rest))
        .map(|(test_value, _)| test_value)
        .sum::<usize>()
}
