use super::Result;
pub fn solve(input: String) -> Result<String> {
    Ok(input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| line.split_once(':'))
        .map(|line| {
            (
                line.0
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Unable to parse the result of line: {line:?}")),
                line.1
                    .split_whitespace()
                    .filter(|val| !val.trim().is_empty())
                    .map(|val| {
                        val.parse::<usize>().unwrap_or_else(|_| {
                            panic!("Unable to parse the operand for line: {line:?}")
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(result, operands)| solution_posible(*result, operands, 1, operands[0]))
        .map(|(result, _)| result)
        .sum::<usize>()
        .to_string())
}

fn solution_posible(result: usize, operands: &[usize], index: usize, sol: usize) -> bool {
    // -ve base case
    if index > operands.len() || (index == operands.len() && result != sol) {
        return false;
    }
    // +ve base case
    if index == operands.len() && result == sol {
        return true;
    }

    // recursive condition
    if solution_posible(result, operands, index + 1, sol + operands[index]) {
        return true;
    }
    if solution_posible(result, operands, index + 1, sol * operands[index]) {
        return true;
    }
    let mut opr2 = operands[index];
    let mut multiplier = 1;
    while opr2 > 0 {
        multiplier *= 10;
        opr2 /= 10;
    }
    if solution_posible(
        result,
        operands,
        index + 1,
        (sol * multiplier) + operands[index],
    ) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13

192: 17 8 14

21037: 9 7 18 13
292: 11 6 16 20";
        let expected = "11387";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
