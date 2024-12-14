fn parse_btn(line: &str) -> (i64, i64) {
    let mut parts = line
        .split(": ")
        .nth(1)
        .expect("Could not find coords")
        .split(", ");

    (
        parts.next().expect("Could not find x")[2..]
            .parse()
            .expect("Could not parse x"),
        parts.next().expect("Could not find y")[2..]
            .parse()
            .expect("Could not parse x"),
    )
}

fn run_game(
    (a_x, a_y): (i64, i64),
    (b_x, b_y): (i64, i64),
    (prize_x, prize_y): (i64, i64),
) -> Option<i64> {
    let det = a_x * b_y - a_y * b_x;

    if det == 0 {
        return None;
    }

    let x_num = prize_x * b_y - prize_y * b_x;
    let y_num = prize_y * a_x - prize_x * a_y;

    if x_num % det == 0 && y_num % det == 0 {
        let x = x_num / det;
        let y = y_num / det;

        Some(x * 3 + y)
    } else {
        None
    }
}

pub fn part1(input: String) -> impl ToString {
    input
        .split("\n\n")
        .filter_map(|games| {
            let mut lines = games.lines();

            let btn_a = parse_btn(lines.next().expect("Could not find x"));
            let btn_b = parse_btn(lines.next().expect("Could not find y"));
            let prize = parse_btn(lines.next().expect("Could not find prize"));

            run_game(btn_a, btn_b, prize)
        })
        .sum::<i64>()
}

pub fn part2(input: String) -> impl ToString {
    input
        .split("\n\n")
        .filter_map(|games| {
            let mut lines = games.lines();

            let btn_a = parse_btn(lines.next().expect("Could not find x"));
            let btn_b = parse_btn(lines.next().expect("Could not find y"));
            let prize = parse_btn(lines.next().expect("Could not find prize"));

            run_game(
                btn_a,
                btn_b,
                (prize.0 + 10000000000000, prize.1 + 10000000000000),
            )
        })
        .sum::<i64>()
}
