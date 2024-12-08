use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use crate::helper::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn part1(input: String) -> impl ToString {
    let mut grid = Grid::from(input);

    let antennas = grid
        .inner()
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, char)| {
                if char == &'.' {
                    None
                } else {
                    Some((
                        *char,
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                    ))
                }
            })
        })
        .fold(
            HashMap::<char, Vec<Point>>::new(),
            |mut acc, (key, point)| {
                if let Some(v) = acc.get_mut(&key) {
                    v.push(point);
                } else {
                    acc.insert(key, vec![point]);
                }
                acc
            },
        );

    let map = grid.inner_mut();

    for lookup in antennas.values() {
        for (idx, lhs) in lookup.iter().enumerate() {
            for rhs in lookup[(idx + 1)..].iter() {
                let diff = *lhs - *rhs;

                let lhs_antinode = if *lhs + diff == *rhs {
                    *lhs - diff
                } else {
                    *lhs + diff
                };

                let rhs_antinode = if *rhs + diff == *lhs {
                    *rhs - diff
                } else {
                    *rhs + diff
                };

                if lhs_antinode.x >= 0
                    && lhs_antinode.x < map.len() as i32
                    && lhs_antinode.y >= 0
                    && lhs_antinode.y < map[0].len() as i32
                {
                    map[lhs_antinode.x as usize][lhs_antinode.y as usize] = '#';
                }

                if rhs_antinode.x >= 0
                    && rhs_antinode.x < map.len() as i32
                    && rhs_antinode.y >= 0
                    && rhs_antinode.y < map[0].len() as i32
                {
                    map[rhs_antinode.x as usize][rhs_antinode.y as usize] = '#';
                }
            }
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|c| c == &&'#').count())
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    let mut grid = Grid::from(input);

    let antennas = grid
        .inner()
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, char)| {
                if char == &'.' {
                    None
                } else {
                    Some((
                        *char,
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                    ))
                }
            })
        })
        .fold(
            HashMap::<char, Vec<Point>>::new(),
            |mut acc, (key, point)| {
                if let Some(v) = acc.get_mut(&key) {
                    v.push(point);
                } else {
                    acc.insert(key, vec![point]);
                }
                acc
            },
        );

    let map = grid.inner_mut();

    for lookup in antennas.values() {
        for (idx, lhs) in lookup.iter().enumerate() {
            for rhs in lookup[(idx + 1)..].iter() {
                map[lhs.x as usize][lhs.y as usize] = '#';

                let diff = *lhs - *rhs;

                let mut i = 1;
                loop {
                    let diff = Point {
                        x: diff.x * i,
                        y: diff.y * i,
                    };

                    let antinode = *lhs + diff;

                    if antinode.x >= 0
                        && antinode.x < map.len() as i32
                        && antinode.y >= 0
                        && antinode.y < map[0].len() as i32
                    {
                        map[antinode.x as usize][antinode.y as usize] = '#';
                    } else {
                        break;
                    }

                    i += 1;
                }

                let mut i = 1;
                loop {
                    let diff = Point {
                        x: diff.x * i,
                        y: diff.y * i,
                    };

                    let antinode = *lhs - diff;

                    if antinode.x >= 0
                        && antinode.x < map.len() as i32
                        && antinode.y >= 0
                        && antinode.y < map[0].len() as i32
                    {
                        map[antinode.x as usize][antinode.y as usize] = '#';
                    } else {
                        break;
                    }

                    i += 1;
                }
            }
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|c| c == &&'#').count())
        .sum::<usize>()
}
