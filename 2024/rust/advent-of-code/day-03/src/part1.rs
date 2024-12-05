use super::Result;
pub fn solve(input: String) -> Result<String> {
    // from the string find all the substring that match expr "mul(num1, num2)"
    let mut results = Vec::new();
    let mut index = 0;
    let len = input.len();
    while index < len - 4 {
        if &input[index..(index + 4)] == "mul(" {
            index += 4;
            parse_mul(&input, &mut index, &mut results);
        } else {
            index += 1;
        }
    }
    Ok(results
        .into_iter()
        .map(|(a, b)| a * b)
        .sum::<isize>()
        .to_string())
}

fn parse_mul(input: &str, index: &mut usize, results: &mut Vec<(isize, isize)>) {
    let start_idx = *index;
    if let Some(val1) = parse_num(input, index) {
        // index will move to ,
        if &input[*index..(*index + 1)] == "," {
            *index += 1;
        } else {
            return;
        }
        // calculate 2nd value
        if let Some(val2) = parse_num(input, index) {
            if &input[*index..(*index + 1)] == ")" {
                *index += 1;
                let end_idx = *index;
                println!("mul({}", &input[start_idx..end_idx]);
            } else {
                return;
            }
            results.push((val1, val2));
        }
    }
}

fn parse_num(input: &str, index: &mut usize) -> Option<isize> {
    let mut num = 0;
    let mut i = 0;
    let ins = input[*index..(*index + 3)].bytes().collect::<Vec<u8>>();
    if ins[i].is_ascii_digit() {
        while i < input.len() && i < 3 && ins[i].is_ascii_digit() {
            num = (num * 10) + (ins[i] - b'0') as isize;
            i += 1;
        }
    } else {
        return None;
    }
    *index += i;
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = "161";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
