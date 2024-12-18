use std::collections::{BTreeSet, HashMap, HashSet};

pub fn part1(input: String) -> impl ToString {
    let grid_size = std::env::var("AOC_SIZE")
        .unwrap_or_else(|_| "70".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_SIZE");

    let simulation_size = std::env::var("AOC_SIM_SIZE")
        .unwrap_or_else(|_| "1024".to_string())
        .parse::<usize>()
        .expect("Could not parse AOC_SIZE");

    let corrupted_spots = input
        .lines()
        .map(|line| {
            let mut parts = line
                .split(",")
                .map(|number| number.parse::<i64>().expect("Could not parse number"));

            (
                parts.next().expect("Could not find lhs"),
                parts.next().expect("Could not find rhs"),
            )
        })
        .take(simulation_size)
        .collect::<HashSet<_>>();

    let exit = (grid_size, grid_size);

    let mut ans = HashMap::new();

    let mut prio = BTreeSet::new();
    prio.insert((0, (0, 0)));

    while let Some((cost, position)) = prio.pop_first() {
        if position == exit {
            continue;
        }

        let next = [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ];

        let next_cost = cost + 1;

        for next in &next {
            if corrupted_spots.contains(next)
                || next.0 < 0
                || next.0 > grid_size
                || next.1 < 0
                || next.1 > grid_size
            {
                continue;
            }

            match ans.get(next) {
                Some(cost) if &next_cost >= cost => {}
                _ => {
                    ans.insert(*next, next_cost);
                    prio.insert((next_cost, *next));
                }
            }
        }
    }

    ans[&exit]
}

pub fn part2(input: String) -> impl ToString {
    let grid_size = std::env::var("AOC_SIZE")
        .unwrap_or_else(|_| "70".to_string())
        .parse::<i64>()
        .expect("Could not parse AOC_SIZE");

    let all_corrupted_spots = input
        .lines()
        .map(|line| {
            let mut parts = line
                .split(",")
                .map(|number| number.parse::<i64>().expect("Could not parse number"));

            (
                parts.next().expect("Could not find lhs"),
                parts.next().expect("Could not find rhs"),
            )
        })
        .collect::<Vec<_>>();

    let exit = (grid_size, grid_size);

    let mut count = all_corrupted_spots.len() / 2;
    let mut inc = all_corrupted_spots.len();

    'outer: loop {
        inc /= 2;

        let corrupted_spots = all_corrupted_spots
            .iter()
            .take(count)
            .cloned()
            .collect::<HashSet<_>>();

        let mut ans = HashMap::new();

        let mut prio = BTreeSet::new();
        prio.insert((0, (0, 0)));

        while let Some((cost, position)) = prio.pop_first() {
            if position == exit {
                // We can still exit the maze, so try a later one
                count += inc.max(1);
                continue 'outer;
            }

            let next = [
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
            ];

            let next_cost = cost + 1;

            for next in &next {
                if corrupted_spots.contains(next)
                    || next.0 < 0
                    || next.0 > grid_size
                    || next.1 < 0
                    || next.1 > grid_size
                {
                    continue;
                }

                match ans.get(next) {
                    Some(cost) if &next_cost >= cost => {}
                    _ => {
                        ans.insert(*next, next_cost);
                        prio.insert((next_cost, *next));
                    }
                }
            }
        }

        // Reaching here means the maze is not solvable
        if inc == 0 {
            // No earlier iteration solves the maze
            let ans = all_corrupted_spots[count - 1];
            break format!("{},{}", ans.0, ans.1);
        } else {
            // Try an earlier one
            count -= inc;
        }
    }
}
