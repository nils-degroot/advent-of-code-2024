use std::collections::HashMap;

pub fn part1(input: String) -> impl ToString {
    let (mut lhs, mut rhs) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split
                    .next()
                    .expect("Could not find lhs")
                    .parse::<usize>()
                    .expect("Could not parse number"),
                split
                    .next()
                    .expect("Could not find rhs")
                    .parse::<usize>()
                    .expect("Could not parse number"),
            )
        })
        .fold(
            (vec![], vec![]),
            |(mut lhs_acc, mut rhs_acc), (lhs, rhs)| {
                lhs_acc.push(lhs);
                rhs_acc.push(rhs);
                (lhs_acc, rhs_acc)
            },
        );

    lhs.sort();
    rhs.sort();

    lhs.into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    let (lhs, rhs) = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split
                    .next()
                    .expect("Could not find lhs")
                    .parse::<usize>()
                    .expect("Could not parse number"),
                split
                    .next()
                    .expect("Could not find rhs")
                    .parse::<usize>()
                    .expect("Could not parse number"),
            )
        })
        .fold(
            (vec![], HashMap::<usize, usize>::new()),
            |(mut lhs_acc, mut rhs_acc), (lhs, rhs)| {
                lhs_acc.push(lhs);

                if let Some(v) = rhs_acc.get_mut(&rhs) {
                    *v += 1;
                } else {
                    rhs_acc.insert(rhs, 1);
                }

                (lhs_acc, rhs_acc)
            },
        );

    lhs.into_iter()
        .map(|n| rhs.get(&n).unwrap_or(&0) * n)
        .sum::<usize>()
}
