use core::panic;

use day_08::{part1::solve as solve1, part2::solve as solve2, solver::solve, Result};

const PART1_INPUT: &str = "res/part1-input.txt";
const PART2_INPUT: &str = "res/part2-input.txt";

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match (args.next(), args.next()) {
        (Some(s), Some(file_name)) => match s.as_str() {
            "1" => solve(&file_name, solve1),
            "2" => solve(&file_name, solve2),
            s => panic!("Input can be (1 or 2). Provided: {s}"),
        },
        (Some(s), None) => match s.as_str() {
            "1" => solve(PART1_INPUT, solve1),
            "2" => solve(PART2_INPUT, solve2),
            s => panic!("Input can be (1 or 2). Provided: {s}"),
        },
        _ => panic!("Input is required. Can be (1 or 2)"),
    }
    Ok(())
}
