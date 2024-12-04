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
        } else {
            for i in 0..values.len() {
                let values: Vec<_> = values
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| index != &i)
                    .map(|(_, value)| value)
                    .collect();
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
                    break;
                }
            }
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
        let expected = "4";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
    #[test]
    fn custom_1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
1 2 3 4 5 32
8 6 4 4 1
21 8 6 5 4 1
1 3 6 7 9";
        let expected = "6";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
