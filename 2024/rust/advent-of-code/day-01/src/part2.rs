use super::Result;
use std::collections::HashMap;
pub fn solve(input: String) -> Result<String> {
    let mut list1 = Vec::new();
    let mut freq_map = HashMap::new();
    for line in input.trim().split('\n') {
        let values: Vec<_> = line.split_whitespace().collect();
        if values.len() != 2 {
            eprintln!("Incorrect input found");
            return Err("Incorrect input found".into());
        }
        let v1 = values[0].parse::<isize>().unwrap();
        let v2 = values[1].parse::<isize>().unwrap();
        list1.push(v1);
        freq_map
            .entry(v2)
            .and_modify(|v| *v += 1_isize)
            .or_insert(1);
    }

    Ok(list1
        .iter()
        .map(|v| v * freq_map.get(v).unwrap_or(&0))
        .sum::<isize>()
        .to_string())
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
        let expected = "31";
        let actual = solve(input.to_owned()).unwrap();
        assert_eq!(expected, actual);
    }
}
