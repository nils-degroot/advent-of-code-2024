use std::collections::{BTreeSet, HashSet};

use crate::helper::Grid;

pub fn part1(input: String) -> impl ToString {
    let grid = Grid::from(input);

    let mut taken = HashSet::new();
    let mut out = 0;

    for x in 0..grid.inner().len() {
        for y in 0..grid.inner()[0].len() {
            if taken.contains(&(x, y)) {
                continue;
            }

            let root_char = grid.inner()[x][y];

            taken.insert((x, y));

            let mut region = HashSet::new();
            region.insert((x, y));

            let mut queue = BTreeSet::new();
            queue.insert((x, y));

            let mut perimiter = 0;

            while let Some((x, y)) = queue.pop_first() {
                if x == 0 || x == grid.inner().len() - 1 {
                    perimiter += 1;
                }
                if y == 0 || y == grid.inner()[0].len() - 1 {
                    perimiter += 1;
                }

                for (pos, char) in grid.neighbors(x, y) {
                    if *char != root_char {
                        perimiter += 1;
                        continue;
                    }
                    if taken.contains(&pos) {
                        continue;
                    }

                    region.insert(pos);
                    taken.insert(pos);
                    queue.insert(pos);
                }
            }

            out += region.len() * perimiter;
        }
    }

    out
}

pub fn part2(input: String) -> impl ToString {
    let grid = Grid::from(input);

    let edge_check = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let mut taken = HashSet::new();
    let mut out = 0;

    for x in 0..grid.inner().len() {
        for y in 0..grid.inner()[0].len() {
            if taken.contains(&(x, y)) {
                continue;
            }

            let root_char = grid.inner()[x][y];

            taken.insert((x, y));

            let mut region = HashSet::new();
            region.insert((x as i32, y as i32));

            let mut queue = BTreeSet::new();
            queue.insert((x, y));

            let mut corners = 0;

            while let Some((x, y)) = queue.pop_first() {
                for ((x, y), char) in grid.neighbors(x, y) {
                    if *char != root_char || taken.contains(&(x, y)) {
                        continue;
                    }

                    region.insert((x as i32, y as i32));
                    taken.insert((x, y));
                    queue.insert((x, y));
                }
            }

            for (x, y) in &region {
                for (x_check, y_check) in &edge_check {
                    if !region.contains(&(*x + x_check, *y))
                        && !region.contains(&(*x, *y + y_check))
                    {
                        corners += 1;
                        continue;
                    }

                    if region.contains(&(*x + x_check, *y))
                        && region.contains(&(*x, *y + y_check))
                        && !region.contains(&(*x + x_check, *y + y_check))
                    {
                        corners += 1;
                    }
                }
            }

            out += region.len() * corners;
        }
    }

    out
}
