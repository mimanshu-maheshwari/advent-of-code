use super::Result;
pub fn solve(input: String) -> Result<String> {
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "1";
        let expected = "1";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
