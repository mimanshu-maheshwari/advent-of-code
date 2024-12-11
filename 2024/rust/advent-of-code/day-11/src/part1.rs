use super::Result;
pub fn solve(input: String) -> Result<String> {
    let mut stones = input
        .split_whitespace()
        .flat_map(|val| val.parse::<usize>())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        // if i < 6 {
        //     println!("{stones:?}");
        // }
        let mut next_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                next_stones.push(1);
                continue;
            }
            let mut len = 0;
            let mut num = stone;
            while num > 0 {
                len += 1;
                num /= 10;
            }
            if (len & 1) == 0 {
                let pow = 10_usize.pow(len / 2);
                next_stones.push(stone / pow);
                next_stones.push(stone % pow);
            } else {
                next_stones.push(stone * 2024);
            }
        }
        stones = next_stones;
    }

    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_1() {
        let input = "125 17";
        // 253000 1 7
        // 253 0 2024 14168
        // 512072 1 20 24 28676032
        // 512 72 2024 2 0 2 4 2867 6032
        // 1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
        // 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
        let expected = "55312";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
