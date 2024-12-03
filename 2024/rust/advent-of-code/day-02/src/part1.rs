use super::Result;
pub fn solve(input: String) -> Result<String> {
    let mut result = 0;
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }
        let values: Vec<_> = line
            .split_whitespace()
            .map(|v| v.trim().parse::<isize>().unwrap())
            .collect();

        if values.is_empty() {
            continue;
        } else if values.len() == 1 {
            result += 1;
        }

        let increasing = values[0] - values[1] <= 0;
        let mut is_safe = true;
        for i in 0..(values.len() - 1) {
            let dif = (values[i] - values[i + 1]).abs();
            if increasing {
                if values[i] >= values[i + 1] || dif > 3 {
                    is_safe = false;
                    break;
                }
            } else if values[i] <= values[i + 1] || dif > 3 {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            result += 1;
        }
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected = "2";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
