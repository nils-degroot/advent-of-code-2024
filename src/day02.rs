pub fn part1(input: String) -> impl ToString {
    input
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect::<Vec<_>>();

            nums.iter().enumerate().skip(1).all(|(idx, now)| {
                let prev = nums[idx - 1];

                if nums[0] < nums[1] {
                    (prev + 1)..=prev + 3
                } else {
                    (prev - 3)..=prev - 1
                }
                .contains(now)
            })
        })
        .count()
}

pub fn part2(input: String) -> impl ToString {
    fn validate(nums: &[i32]) -> bool {
        nums.iter().enumerate().skip(1).all(|(idx, now)| {
            let prev = nums[idx - 1];

            if nums[0] < nums[1] {
                (prev + 1)..=prev + 3
            } else {
                (prev - 3)..=prev - 1
            }
            .contains(now)
        })
    }

    input
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect::<Vec<_>>();

            nums.iter().enumerate().any(|(idx, _)| {
                let mut now = vec![];
                now.extend_from_slice(&nums[..idx]);
                now.extend_from_slice(&nums[(idx + 1)..]);

                validate(&now)
            })
        })
        .count()
}
