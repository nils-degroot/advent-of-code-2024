use std::collections::HashSet;

use crate::helper::Grid;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::U => Direction::R,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
            Direction::R => Direction::D,
        }
    }
}

pub fn part1(input: String) -> impl ToString {
    let mut grid = Grid::from(input);

    let (mut x, mut y) = grid
        .inner()
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, char)| (char == &'^').then_some((x, y)))
        })
        .expect("Could not find guard");

    let map = grid.inner_mut();

    let mut direction = Direction::U;

    'outer: loop {
        map[x][y] = 'X';

        if !(1..(map.len() - 1)).contains(&x) || !(1..(map[0].len() - 1)).contains(&y) {
            break 'outer;
        }

        loop {
            let next = match direction {
                Direction::U => (x - 1, y),
                Direction::D => (x + 1, y),
                Direction::L => (x, y - 1),
                Direction::R => (x, y + 1),
            };

            if map[next.0][next.1] == '#' {
                direction = direction.next();
            } else {
                x = next.0;
                y = next.1;
                break;
            }
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|char| char == &&'X').count())
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    fn infinite(mut x: usize, mut y: usize, mut grid: Grid<char>) -> bool {
        let map = grid.inner_mut();
        let mut direction = Direction::U;
        let mut visited = HashSet::new();

        'outer: loop {
            map[x][y] = 'X';

            if !(1..(map.len() - 1)).contains(&x) || !(1..(map[0].len() - 1)).contains(&y) {
                break 'outer false;
            }

            loop {
                let next = match direction {
                    Direction::U => (x - 1, y),
                    Direction::D => (x + 1, y),
                    Direction::L => (x, y - 1),
                    Direction::R => (x, y + 1),
                };

                if map[next.0][next.1] == '#' {
                    direction = direction.next();
                } else {
                    x = next.0;
                    y = next.1;
                    break;
                }
            }

            if visited.contains(&(x, y, direction.clone())) {
                break true;
            }

            map[x][y] = '^';
            visited.insert((x, y, direction.clone()));
        }
    }

    let grid = Grid::from(input);

    let (x, y) = grid
        .inner()
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, char)| (char == &'^').then_some((x, y)))
        })
        .expect("Could not find guard");

    let mut out = 0;

    for (bx, row) in grid.inner().iter().enumerate() {
        for (by, char) in row.iter().enumerate() {
            if char == &'#' || char == &'^' {
                continue;
            }

            let mut grid = grid.clone();
            grid.inner_mut()[bx][by] = '#';

            if infinite(x, y, grid) {
                out += 1;
            }
        }
    }

    out
}
