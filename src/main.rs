use clap::Parser;

use std::path::PathBuf;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod helper;

#[derive(Debug, Parser)]
struct Args {
    /// Day to execute
    day: usize,
    /// Filter on part
    #[clap(short, long)]
    part: Option<usize>,
    /// Trim the output
    #[clap(short, long)]
    trim: bool,
    /// Use a different file as input
    #[clap(short, long)]
    input: Option<String>,
}

macro_rules! matcher {
    ( $args:expr, $( $day:expr => $module:ident ),+ $(,)? ) => {{
        let args = $args;

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input")
            .join(format!("{:02}", args.day))
            .join(args.input.as_ref().map_or("input.txt", |v| v));

        let input = std::fs::read_to_string(path).expect("Failed to read input file");

        match args.day {
            $(
                $day => {
                    let part1 = if let None | Some(1) = args.part {
                        Some($module::part1(input.clone()).to_string())
                    } else {
                        None
                    };

                    let part2 = if let None | Some(2) = args.part {
                        Some($module::part2(input.clone()).to_string())
                    } else {
                        None
                    };

                    (part1, part2)
                }
            )+
            _ => panic!("Unsupported day `{}`", args.day)
        }}
    };
}

fn main() {
    let args = Args::parse();

    let (part1, part2) = matcher! { &args,
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        6 => day06,
        7 => day07,
        8 => day08,
        9 => day09,
        10 => day10,
        11 => day11,
        12 => day12,
        13 => day13,
        14 => day14,
        15 => day15,
        16 => day16,
        17 => day17,
        18 => day18,
    };

    match (args.trim, part1, part2) {
        (true, Some(part1), Some(part2)) => println!("{part1}\n{part2}"),
        (true, Some(part1), None) => println!("{part1}"),
        (true, None, Some(part2)) => println!("{part2}"),
        (false, Some(part1), Some(part2)) => println!("Part 1: `{part1}`\nPart 2: `{part2}`"),
        (false, Some(part1), None) => println!("Part 1: `{part1}`"),
        (false, None, Some(part2)) => println!("Part 2: `{part2}`"),
        _ => panic!("No part was selected"),
    }
}
