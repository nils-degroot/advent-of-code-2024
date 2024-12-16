use std::collections::{BTreeSet, HashMap, HashSet};

use crate::helper::Grid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::U => Direction::R,
            Direction::R => Direction::D,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
        }
    }

    fn counterclockwise(&self) -> Self {
        match self {
            Direction::R => Direction::U,
            Direction::D => Direction::R,
            Direction::L => Direction::D,
            Direction::U => Direction::L,
        }
    }
}

pub fn part1(input: String) -> impl ToString {
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
    ans.insert((start, Direction::R), 0);

    let mut prio = BTreeSet::new();
    prio.insert((0, start, Direction::R));

    while let Some((cost, position, direction)) = prio.pop_first() {
        if position == end {
            continue;
        }

        let next = [
            (position, direction.clockwise(), cost + 1000),
            (position, direction.counterclockwise(), cost + 1000),
            (
                match direction {
                    Direction::U => (position.0 - 1, position.1),
                    Direction::R => (position.0, position.1 + 1),
                    Direction::D => (position.0 + 1, position.1),
                    Direction::L => (position.0, position.1 - 1),
                },
                direction,
                cost + 1,
            ),
        ];

        for (new_position, new_direction, new_cost) in next {
            if grid.inner()[new_position.0][new_position.1] == '#' {
                continue;
            }

            match ans.get(&(new_position, new_direction)) {
                Some(cost) if &new_cost >= cost => {}
                _ => {
                    ans.insert((new_position, new_direction), new_cost);
                    prio.insert((new_cost, new_position, new_direction));
                }
            }
        }
    }

    ans.iter()
        .filter(|((pos, _), _)| pos == &end)
        .map(|(_, cost)| *cost)
        .min()
        .expect("Everything went wrong")
}

pub fn part2(input: String) -> impl ToString {
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
    ans.insert((start, Direction::R), 0);

    let mut prio = BTreeSet::new();
    prio.insert((0, start, Direction::R));

    while let Some((cost, position, direction)) = prio.pop_first() {
        if position == end {
            continue;
        }

        let next = [
            (position, direction.clockwise(), cost + 1000),
            (position, direction.counterclockwise(), cost + 1000),
            (
                match direction {
                    Direction::U => (position.0 - 1, position.1),
                    Direction::R => (position.0, position.1 + 1),
                    Direction::D => (position.0 + 1, position.1),
                    Direction::L => (position.0, position.1 - 1),
                },
                direction,
                cost + 1,
            ),
        ];

        for (new_position, new_direction, new_cost) in next {
            if grid.inner()[new_position.0][new_position.1] == '#' {
                continue;
            }

            match ans.get(&(new_position, new_direction)) {
                Some(cost) if &new_cost >= cost => {}
                _ => {
                    ans.insert((new_position, new_direction), new_cost);
                    prio.insert((new_cost, new_position, new_direction));
                }
            }
        }
    }

    let lowest_score = ans
        .iter()
        .filter(|((pos, _), _)| pos == &end)
        .map(|(_, cost)| *cost)
        .min()
        .expect("Everything went wrong");

    let mut prio = BTreeSet::new();
    prio.insert((lowest_score, end, Direction::U));
    prio.insert((lowest_score, end, Direction::L));
    prio.insert((lowest_score, end, Direction::R));
    prio.insert((lowest_score, end, Direction::D));

    let mut positions = HashSet::new();
    positions.insert(start);
    positions.insert(end);

    // Walk back in reverse, checking if the score for the new tile matches up to where we went
    // before.
    while let Some((cost, position, direction)) = prio.pop_first() {
        if position == start {
            continue;
        }

        let next = [
            (position, direction.clockwise(), cost - 1000),
            (position, direction.counterclockwise(), cost - 1000),
            (
                match direction {
                    Direction::U => (position.0 + 1, position.1),
                    Direction::R => (position.0, position.1 - 1),
                    Direction::D => (position.0 - 1, position.1),
                    Direction::L => (position.0, position.1 + 1),
                },
                direction,
                cost - 1,
            ),
        ];

        for (new_position, new_direction, new_cost) in next {
            if grid.inner()[new_position.0][new_position.1] == '#' || new_cost < 0 {
                continue;
            }

            match ans.get(&(new_position, new_direction)) {
                Some(old_cost) if old_cost == &new_cost => {
                    positions.insert(new_position);
                    prio.insert((new_cost, new_position, new_direction));
                }
                _ => {}
            }
        }
    }

    positions.len()
}
