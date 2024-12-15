use crate::helper::Grid;

fn parse_input(input: String) -> (Grid<char>, String) {
    let mut parts = input.split("\n\n");

    let grid = Grid::from(parts.next().expect("Could not find grid").to_string());

    let movements = parts
        .next()
        .expect("Could not find movements")
        .trim_end()
        .lines()
        .collect::<String>();

    (grid, movements)
}

fn robot_coords(map: &[Vec<char>]) -> (i32, i32) {
    map.iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .enumerate()
                .find_map(|(y, char)| (char == &'@').then_some((x as i32, y as i32)))
        })
        .expect("Could not find robot")
}

pub fn part1(input: String) -> impl ToString {
    fn attempt_move(map: &mut [Vec<char>], pos: (i32, i32), movement: (i32, i32)) -> (i32, i32) {
        let old_char = map[pos.0 as usize][pos.1 as usize];
        let new_pos = (pos.0 + movement.0, pos.1 + movement.1);

        // This is a wall, just return early
        if map[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            return pos;
        }

        // When we reach a box, attempt to move the box
        if map[new_pos.0 as usize][new_pos.1 as usize] == 'O' {
            let box_next_pos = attempt_move(map, new_pos, movement);

            // Could not move the box, so the robot cannot move either
            if box_next_pos == new_pos {
                return pos;
            } else {
                map[new_pos.0 as usize][new_pos.1 as usize] = old_char;
                map[pos.0 as usize][pos.1 as usize] = '.';
                return new_pos;
            }
        }

        // Space is empty, so just move
        map[new_pos.0 as usize][new_pos.1 as usize] = old_char;
        map[pos.0 as usize][pos.1 as usize] = '.';

        new_pos
    }

    let (mut grid, movements) = parse_input(input);

    let mut pos = robot_coords(grid.inner());

    let map = grid.inner_mut();

    for movement in movements.chars() {
        let movement = match movement {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Got invalid movement: {movement}"),
        };

        pos = attempt_move(map, pos, movement);
    }

    map.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, char)| (char == &'O').then_some(100 * x + y))
        })
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    fn could_move(map: &[Vec<char>], pos: (i32, i32), movement: (i32, i32)) -> bool {
        let new_pos = (pos.0 + movement.0, pos.1 + movement.1);

        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '#' => false,
            '[' | ']' => {
                let (box_lhs, box_rhs) = if map[new_pos.0 as usize][new_pos.1 as usize] == '[' {
                    (new_pos, (new_pos.0, new_pos.1 + 1))
                } else {
                    ((new_pos.0, new_pos.1 - 1), new_pos)
                };

                match movement {
                    // Up of down
                    (-1, 0) | (1, 0) => {
                        could_move(map, box_lhs, movement) && could_move(map, box_rhs, movement)
                    }
                    // Left
                    (0, -1) => could_move(map, box_lhs, movement),
                    // Right
                    (0, 1) => could_move(map, box_rhs, movement),
                    _ => panic!(),
                }
            }
            '.' => true,
            _ => panic!("Missiles, fire"),
        }
    }

    fn do_move(map: &mut [Vec<char>], pos: (i32, i32), movement: (i32, i32)) {
        let new_pos = (pos.0 + movement.0, pos.1 + movement.1);

        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '[' if movement == (0, 1) => {
                do_move(map, (new_pos.0, new_pos.1 + 1), movement);
                do_move(map, new_pos, movement);
            }
            '[' => {
                do_move(map, new_pos, movement);
                do_move(map, (new_pos.0, new_pos.1 + 1), movement);
            }
            ']' if movement == (0, 1) => {
                do_move(map, new_pos, movement);
                do_move(map, (new_pos.0, new_pos.1 - 1), movement);
            }
            ']' => {
                do_move(map, (new_pos.0, new_pos.1 - 1), movement);
                do_move(map, new_pos, movement);
            }
            '.' => {}
            _ => {
                do_move(map, new_pos, movement);
            }
        }

        let old_char = map[pos.0 as usize][pos.1 as usize];

        map[pos.0 as usize][pos.1 as usize] = '.';
        map[new_pos.0 as usize][new_pos.1 as usize] = old_char;
    }

    let (grid, movements) = parse_input(input);

    let mut map = grid
        .inner()
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|char| match char {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!("Fire fire"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pos = robot_coords(&map);

    for movement in movements.chars() {
        let movement = match movement {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Got invalid movement: {movement}"),
        };

        if could_move(&map, pos, movement) {
            do_move(&mut map, pos, movement);
            pos = (pos.0 + movement.0, pos.1 + movement.1);
        }
    }

    map.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, char)| (char == &'[').then_some(100 * x + y))
        })
        .sum::<usize>()
}
