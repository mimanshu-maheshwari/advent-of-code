use super::Result;
pub fn solve(input: String) -> Result<String> {
    let mut index = 0;
    let mut flip = false;
    let mut input: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|line| line.bytes().map(|b| (b - b'0') as usize))
        // .inspect(|v| {
        //     dbg!(v);
        // })
        .flat_map(|v| {
            // println!("before: {index}");
            flip = !flip;
            vec![
                if flip {
                    let val = Some(index);
                    index += 1;
                    // println!("after: {index}");
                    val
                } else {
                    None
                };
                v
            ]
            .into_iter()
        })
        .collect();
    // println!("{input:?}");

    let mut start = 0;
    let mut end = input.len() - 1;
    while start < end {
        while start < end && input[start].is_some() {
            start += 1;
        }
        while start < end && input[end].is_none() {
            end -= 1;
        }
        if input[start].is_none() {
            input.swap(start, end);
            end -= 1;
        }
        start += 1;
    }
    // println!("{input:?}");
    Ok(input
        .iter()
        .enumerate()
        .map(|(idx, val)| if let Some(val) = val { val * idx } else { 0 })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "2333133121414131402";
        let expected = "1928";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
