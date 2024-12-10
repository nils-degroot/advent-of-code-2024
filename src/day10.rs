use std::collections::HashSet;

use crate::helper::Grid;

pub fn part1(input: String) -> impl ToString {
    fn check_point(
        grid: &Grid<char>,
        reached: &mut HashSet<(usize, usize)>,
        (x, y): (usize, usize),
        level: char,
    ) {
        let level = level.to_digit(10).expect("Could not parse digit");

        if level == 9 {
            reached.insert((x, y));
        } else {
            grid.neighbors(x, y)
                .filter(|(_, n_level)| {
                    if n_level == &&'.' {
                        false
                    } else {
                        n_level.to_digit(10).expect("Could not parse digit") == level + 1
                    }
                })
                .for_each(|(point, level)| check_point(grid, reached, point, *level));
        }
    }

    let grid = Grid::from(input);

    grid.inner()
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(y, char)| if char == &'0' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|point| {
            let mut map = HashSet::new();
            check_point(&grid, &mut map, point, '0');
            map.len()
        })
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    fn check_point(grid: &Grid<char>, (x, y): (usize, usize), level: char) -> usize {
        let level = level.to_digit(10).expect("Could not parse digit");

        if level == 9 {
            1
        } else {
            grid.neighbors(x, y)
                .filter(|(_, n_level)| {
                    if n_level == &&'.' {
                        false
                    } else {
                        n_level.to_digit(10).expect("Could not parse digit") == level + 1
                    }
                })
                .map(|(point, level)| check_point(grid, point, *level))
                .sum()
        }
    }

    let grid = Grid::from(input);

    grid.inner()
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(y, char)| if char == &'0' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|point| check_point(&grid, point, '0'))
        .sum::<usize>()
}
