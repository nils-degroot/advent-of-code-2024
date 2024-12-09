pub fn part1(input: String) -> impl ToString {
    let mut blocks = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(idx, char)| {
            let digit = char.to_digit(10).expect("Could not parse digit");

            if idx % 2 == 0 {
                (0..digit).map(|_| Some(idx / 2)).collect::<Vec<_>>()
            } else {
                (0..digit).map(|_| None).collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();

    let mut head = 0;
    let mut tail = blocks.len() - 1;

    while head < tail {
        while blocks[head].is_some() {
            head += 1;
        }
        while blocks[tail].is_none() {
            tail -= 1;
        }

        blocks.swap(head, tail);
    }

    blocks
        .iter()
        .flatten()
        .enumerate()
        .map(|(idx, num)| idx * *num)
        .sum::<usize>()
}

pub fn part2(input: String) -> impl ToString {
    let mut blocks = input
        .trim()
        .chars()
        .enumerate()
        .map(|(idx, char)| {
            let digit = char.to_digit(10).expect("Could not parse digit");

            if idx % 2 == 0 {
                (digit, Some(idx / 2))
            } else {
                (digit, None)
            }
        })
        .collect::<Vec<_>>();

    let mut head = 0;

    'outer: while head < blocks.len() {
        while blocks[head].1.is_some() {
            head += 1;

            if head >= blocks.len() {
                break 'outer;
            }
        }

        let (head_count, _) = blocks[head];

        let res = blocks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(idx, (count, id))| {
                if count <= &head_count && idx > head {
                    id.map(|_id| (idx, *count))
                } else {
                    None
                }
            });

        if let Some((tail, count)) = res {
            if head_count == count {
                blocks.swap(head, tail);
            } else {
                blocks.swap(head, tail);
                blocks[tail] = (count, None);
                blocks.insert(head + 1, (head_count - count, None));
            }
        } else {
            head += 1;
        }
    }

    blocks
        .iter()
        .flat_map(|(count, id)| {
            if let Some(id) = id {
                (0..*count).map(|_| Some(id)).collect::<Vec<_>>()
            } else {
                (0..*count).map(|_| None).collect::<Vec<_>>()
            }
        })
        .enumerate()
        .filter_map(|(idx, v)| v.map(|v| (idx, v)))
        .map(|(idx, num)| idx * *num)
        .sum::<usize>()
}
