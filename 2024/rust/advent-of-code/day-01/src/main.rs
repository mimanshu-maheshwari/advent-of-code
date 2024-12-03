use day_01::{part1::solve as solve1, part2::solve as solve2, Result};
use std::{fs::File, io::Read};

const PART1_INPUT: &str = "res/part1-input.txt";
const PART2_INPUT: &str = "res/part1-input.txt";

fn main() -> Result<()> {
    println!("Part1: ");
    solve(PART1_INPUT, solve1);
    println!();
    println!("Part2: ");
    solve(PART2_INPUT, solve2);
    Ok(())
}

fn read_file_to_string(file_name: &str) -> Result<String> {
    let mut file = File::options().read(true).open(file_name)?;
    let mut input = String::new();
    let _bytes_read = file.read_to_string(&mut input)?;
    Ok(input)
}

fn solve<F>(file_name: &str, solver: F)
where
    F: Fn(String) -> Result<String>,
{
    match read_file_to_string(file_name) {
        Ok(input) => match solver(input) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("Error thrown by solver: {err}"),
        },
        Err(err) => eprintln!("Error thrown by file reader: {err}"),
    }
}
