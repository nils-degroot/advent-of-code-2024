pub fn part1(input: String) -> impl ToString {
    let chars = input.chars().collect::<Vec<_>>();

    let mut idx = 0;
    let mut sum = 0;

    while idx < chars.len() {
        if input[idx..].starts_with("mul(") {
            idx += 4;

            let lhs = chars[idx..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            idx += lhs.len();

            if chars[idx] != ',' {
                continue;
            }

            idx += 1;

            let rhs = chars[idx..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            idx += rhs.len();

            if chars[idx] != ')' {
                continue;
            }

            sum += lhs.parse::<usize>().expect("Failed to parse lhs")
                * rhs.parse::<usize>().expect("Failed to parse rhs")
        }

        idx += 1;
    }

    sum
}

pub fn part2(input: String) -> impl ToString {
    let chars = input.chars().collect::<Vec<_>>();

    let mut enable = true;
    let mut idx = 0;
    let mut sum = 0;

    while idx < chars.len() {
        if input[idx..].starts_with("mul(") && enable {
            idx += 4;

            let lhs = chars[idx..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            idx += lhs.len();

            if chars[idx] != ',' {
                continue;
            }

            idx += 1;

            let rhs = chars[idx..]
                .iter()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            idx += rhs.len();

            if chars[idx] != ')' {
                continue;
            }

            sum += lhs.parse::<usize>().expect("Failed to parse lhs")
                * rhs.parse::<usize>().expect("Failed to parse rhs")
        } else if input[idx..].starts_with("do()") {
            enable = true;
            idx += 3;
        } else if input[idx..].starts_with("don't()") {
            enable = false;
            idx += 6;
        }

        idx += 1;
    }

    sum
}
