use std::collections::{BTreeSet, HashMap};

use crate::helper::Grid;

fn solve(input: String, cheat_distance: i32) -> usize {
    let grid = Grid::from(input);

    let start = grid
        .inner()
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, char)| (char == &'S').then_some((x, y)))
        })
        .expect("Could not find start");

    let end = grid
        .inner()
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, char)| (char == &'E').then_some((x, y)))
        })
        .expect("Could not find end");

    let mut ans = HashMap::new();
    ans.insert(start, 0usize);

    let mut prio = BTreeSet::new();
    prio.insert((0usize, start));

    while let Some((cost, position)) = prio.pop_first() {
        if position == end {
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
            if grid.inner()[next.0][next.1] == '#' {
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

    let manhatten_matrix = (-cheat_distance..=cheat_distance)
        .flat_map(|idx| {
            let diff = idx.abs().abs_diff(cheat_distance) as i32;
            (-diff..=diff).map(|y| (idx, y)).collect::<Vec<_>>()
        })
        .filter(|(x, y)| !((-1..=1).contains(x) && (-1..=1).contains(y)))
        .collect::<Vec<_>>();

    let mut cheats = vec![];

    for (point, start_cost) in ans.iter() {
        if point == &end {
            continue;
        }

        for mutation in &manhatten_matrix {
            let end = (
                (point.0 as i32 + mutation.0) as usize,
                (point.1 as i32 + mutation.1) as usize,
            );

            let end_cost = match ans.get(&end) {
                Some(cost) => *cost,
                None => continue,
            };

            let length = (mutation.0.abs() + mutation.1.abs()) as usize;

            if start_cost + length < end_cost {
                let diff = start_cost.abs_diff(end_cost) - length;
                cheats.push(diff);
            }
        }
    }

    cheats.iter().filter(|cheat| cheat >= &&100).count()
}

pub fn part1(input: String) -> impl ToString {
    solve(input, 2)
}

pub fn part2(input: String) -> impl ToString {
    solve(input, 20)
}
