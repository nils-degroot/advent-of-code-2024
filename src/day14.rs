fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let position = parts.next().expect("Could not find position");
            let velocity = parts.next().expect("Could not find velocity");

            (parse_coords(position), parse_coords(velocity))
        })
        .collect::<Vec<_>>()
}

fn parse_coords(piece: &str) -> (i64, i64) {
    let mut parts = piece.split(&['=', ',']).skip(1);

    let (y, x) = (
        parts
            .next()
            .expect("Could not find coord part")
            .parse()
            .expect("Could not parse coord"),
        parts
            .next()
            .expect("Could not find coord part")
            .parse()
            .expect("Could not parse coord"),
    );

    (x, y)
}

pub fn part1(input: String) -> impl ToString {
    let height = std::env::var("AOC_HEIGHT")
        .unwrap_or_else(|_| "103".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_HEIGHT");

    let width = std::env::var("AOC_WIDTH")
        .unwrap_or_else(|_| "101".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_WIDTH");

    let input = parse_input(&input)
        .into_iter()
        .map(|(robot, velocity)| {
            (
                (robot.0 + velocity.0 * 100).rem_euclid(height),
                (robot.1 + velocity.1 * 100).rem_euclid(width),
            )
        })
        .collect::<Vec<_>>();

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for x in 0..height {
        for y in 0..width {
            if x == height / 2 || y == width / 2 {
                continue;
            }

            let count = input.iter().filter(|p| (p.0, p.1) == (x, y)).count();

            match (x < height / 2, y < width / 2) {
                (true, true) => tl += count,
                (true, false) => tr += count,
                (false, true) => bl += count,
                (false, false) => br += count,
            }
        }
    }

    tl * tr * bl * br
}

pub fn part2(input: String) -> impl ToString {
    let height = std::env::var("AOC_HEIGHT")
        .unwrap_or_else(|_| "103".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_HEIGHT");

    let width = std::env::var("AOC_WIDTH")
        .unwrap_or_else(|_| "101".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_WIDTH");

    let mut input = parse_input(&input);

    let mut acc = vec![];

    // Take a random point to compare to
    let det = (height / 2, width / 2);

    // Assuming the tree is in the first 10_000
    for idx in 0..=10_000 {
        // Move the robots
        input = input
            .into_iter()
            .map(|(robot, velocity)| {
                (
                    (
                        (robot.0 + velocity.0).rem_euclid(height),
                        (robot.1 + velocity.1).rem_euclid(width),
                    ),
                    velocity,
                )
            })
            .collect::<Vec<_>>();

        // Calculate the distance between det and the bots
        let distances = input
            .iter()
            .map(|(r, _)| r)
            .map(|r| (((det.0 - r.0).pow(2) + (det.1 - r.1).pow(2)) as f64).sqrt())
            .collect::<Vec<_>>();

        // Take the average
        let avg_distance = distances.iter().sum::<f64>() / distances.len() as f64;
        acc.push((idx + 1, avg_distance));
    }

    acc.iter()
        .min_by(|(_, l), (_, r)| l.partial_cmp(r).expect("Failed to cmp"))
        .expect("Could not find anything")
        .0
}
