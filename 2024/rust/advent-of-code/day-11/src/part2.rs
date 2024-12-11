use std::collections::HashMap;

use super::Result;

pub fn solve(input: String) -> Result<String> {
    let mut dp = HashMap::new();
    let input = input
        .split_whitespace()
        .flat_map(|val| val.parse::<usize>())
        .collect::<Vec<_>>();
    let mut result = 0;
    for stone in input {
        result += dfs(&mut dp, (stone, 75));
    }
    Ok(result.to_string())
}

fn dfs(dp: &mut HashMap<(usize, usize), usize>, (stone, blink): (usize, usize)) -> usize {
    if let Some(&stones) = dp.get(&(stone, blink)) {
        return stones;
    }

    let ret = 
    // we completed the number of blinks
    if blink == 0 {
        1
    }
    // stone is 0
    else if stone == 0 {
        dfs(dp, (1, blink - 1))
    }
    // is even length
    else if (stone.to_string().len() & 1) == 0 {
        let mut len = 0;
        let mut num = stone;
        while num > 0 {
            len += 1;
            num /= 10;
        }
        let pow = 10_usize.pow(len / 2);
        let left = dfs(dp, ((stone / pow), blink - 1));
        let right = dfs(dp, ((stone % pow), blink - 1));
        left + right
    }
    // lenght is odd
    else {
        dfs(dp, (stone * 2024, blink - 1))
    };
    dp.insert((stone, blink), ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    #[ignore = "is only valid for 25 iters"]
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
