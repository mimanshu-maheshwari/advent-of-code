use std::{error, result};
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
pub mod part1;
pub mod part2;
