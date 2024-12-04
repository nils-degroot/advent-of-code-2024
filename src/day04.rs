pub fn part1(input: String) -> impl ToString {
    fn instances(input: String) -> usize {
        (0..input.len())
            .filter(|idx| input[*idx..].starts_with("XMAS"))
            .count()
    }

    fn grid_get(grid: &[Vec<char>], x: usize, y: usize) -> Option<char> {
        grid.get(x).and_then(|line| line.get(y)).cloned()
    }

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // RU -> LD
    // LD -> RU

    let mut lines = vec![];

    // L -> R
    lines.extend(
        grid.iter()
            .flat_map(|line| {
                vec![
                    line.iter().collect::<String>(),
                    line.iter().rev().collect::<String>(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    // U -> D
    lines.extend(
        (0..grid[0].len())
            .flat_map(|idx| {
                vec![
                    grid.iter().map(|line| line[idx]).collect::<String>(),
                    grid.iter().map(|line| line[idx]).rev().collect::<String>(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    // LU -> RD
    for mut x in 0..grid.len() {
        let mut y = 0;

        let mut acc = String::new();

        while let Some(c) = grid_get(&grid, x, y) {
            acc.push(c);
            x += 1;
            y += 1;
        }

        lines.push(acc.clone());
        lines.push(acc.chars().rev().collect::<String>());
    }
    for mut y in 1..grid[0].len() {
        let mut x = 0;

        let mut acc = String::new();

        while let Some(c) = grid_get(&grid, x, y) {
            acc.push(c);
            x += 1;
            y += 1;
        }

        lines.push(acc.clone());
        lines.push(acc.chars().rev().collect::<String>());
    }

    // LD -> RU
    for mut x in 0..grid.len() {
        let mut y = 0;

        let mut acc = String::new();

        while let Some(c) = grid_get(&grid, x, y) {
            acc.push(c);

            if x == 0 {
                break;
            }

            x -= 1;
            y += 1;
        }

        lines.push(acc.clone());
        lines.push(acc.chars().rev().collect::<String>());
    }
    for mut y in 1..grid[0].len() {
        let mut x = grid.len() - 1;

        let mut acc = String::new();

        while let Some(c) = grid_get(&grid, x, y) {
            acc.push(c);

            if x == 0 {
                break;
            }

            x -= 1;
            y += 1;
        }

        lines.push(acc.clone());
        lines.push(acc.chars().rev().collect::<String>());
    }

    lines
        .into_iter()
        .filter(|line| line.len() >= 4)
        .map(instances)
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for x in 0..(grid.len() - 2) {
        for y in 0..(grid[0].len() - 2) {
            let lu = [grid[x][y], grid[x + 1][y + 1], grid[x + 2][y + 2]]
                .iter()
                .collect::<String>();

            let ru = [grid[x][y + 2], grid[x + 1][y + 1], grid[x + 2][y]]
                .iter()
                .collect::<String>();

            if (&lu == "MAS" || &lu == "SAM") && (&ru == "MAS" || &ru == "SAM") {
                count += 1;
            }
        }
    }

    count
}
