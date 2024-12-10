use super::Result;
use std::{fs::File, io::Read};

fn read_file_to_string(file_name: &str) -> Result<String> {
    let mut file = File::options().read(true).open(file_name)?;
    let mut input = String::new();
    let _bytes_read = file.read_to_string(&mut input)?;
    Ok(input)
}

pub fn solve<F>(file_name: &str, solver: F)
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
