use super::Result;
pub fn solve(input: String) -> Result<String> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.trim().split('\n') {
        let values: Vec<_> = line.split_whitespace().collect();
        if values.len() != 2 {
            eprintln!("Incorrect input found");
            return Err("Incorrect input found".into());
        }
        list1.push(values[0].parse::<isize>().unwrap());
        list2.push(values[1].parse::<isize>().unwrap());
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let value = list1
        .iter()
        .zip(list2.iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum::<isize>();
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected = "11";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
