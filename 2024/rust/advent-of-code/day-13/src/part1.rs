use core::panic;

use super::Result;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Game {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

impl Game {
    fn new(input: &str) -> Self {
        let inputs = input.trim().split('\n').collect::<Vec<_>>();
        let mut game = Game {
            button_a: Point::zero(),
            button_b: Point::zero(),
            prize: Point::zero(),
        };
        if inputs.len() != 3 {
            panic!("Input for game is incorrect: {inputs:?}");
        }
        if let Some(line) = inputs[0].trim().strip_prefix("Button A: ") {
            game.button_a = Game::parse_button(line.trim());
        } else {
            panic!("Button A not found")
        }
        if let Some(line) = inputs[1].trim().strip_prefix("Button B: ") {
            game.button_b = Game::parse_button(line.trim());
        } else {
            panic!("Button B not found")
        }
        if let Some(line) = inputs[2].trim().strip_prefix("Prize: ") {
            game.prize = Game::parse_prize(line.trim());
        } else {
            panic!("Prize not found")
        }
        game
    }

    fn parse_prize(line: &str) -> Point {
        if let Some((x, y)) = line.trim().split_once(", ") {
            let x = if let Some(val) = x.strip_prefix("X=") {
                val.parse::<f64>().unwrap()
            } else {
                panic!("Unable to find x value");
            };
            let y = if let Some(val) = y.strip_prefix("Y=") {
                val.parse::<f64>().unwrap()
            } else {
                panic!("Unable to find y value");
            };
            Point::new(x, y)
        } else {
            panic!("Invalid values of button: {line}");
        }
    }

    fn parse_button(line: &str) -> Point {
        if let Some((x, y)) = line.trim().split_once(", ") {
            let x = if let Some(val) = x.strip_prefix("X+") {
                val.parse::<f64>().unwrap()
            } else {
                panic!("Unable to find x value");
            };
            let y = if let Some(val) = y.strip_prefix("Y+") {
                val.parse::<f64>().unwrap()
            } else {
                panic!("Unable to find y value");
            };
            Point::new(x, y)
        } else {
            panic!("Invalid values of button: {line}");
        }
    }

    // find minimum tokens required to reach the prize using game button a and b
    // 3 tokens to push A
    // 1 token to push B
    fn play(&self) -> Option<f64> {
        // let mut dp = HashMap::new();
        // let value = None;
        // let value = self.dfs(self.prize, &mut dp);
        let a = (self.prize.x * self.button_b.y - self.prize.y * self.button_b.x)
            / (self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x);
        let b = (self.prize.x - self.button_a.x * a) / self.button_b.x;
        if (a % 1.0 == 0.0 && b % 1.0 == 0.0) && (a < 101.0 && b < 101.0) {
            return Some(a * 3.0 + b);
        }
        None
    }

    // fn _dfs(&self, curr_sum: Point, dp: &mut HashMap<Point, Option<f64>>) -> Option<f64> {
    //     println!("{curr_sum:?}");
    //     if curr_sum == Point::new(0, 0) {
    //         return Some(0);
    //     }
    //
    //     if let Some(sum) = dp.get(&curr_sum) {
    //         return *sum;
    //     }
    //
    //     // wrapping sub
    //     if curr_sum > self.prize {
    //         return None;
    //     }
    //
    //     // add button a
    //     let value_a = if self.button_a.x <= curr_sum.x || self.button_a.y <= curr_sum.y {
    //         self._dfs(
    //             Point::new(curr_sum.x - self.button_a.x, curr_sum.y - self.button_a.y),
    //             dp,
    //         )
    //     } else {
    //         None
    //     };
    //
    //     // add button b
    //     let value_b = if self.button_b.x <= curr_sum.x || self.button_b.y <= curr_sum.y {
    //         self._dfs(
    //             Point::new(curr_sum.x + self.button_b.x, curr_sum.y + self.button_b.y),
    //             dp,
    //         )
    //     } else {
    //         None
    //     };
    //     dp.insert(
    //         curr_sum,
    //         match (value_a, value_b) {
    //             (Some(a), Some(b)) => Some(std::cmp::min(3 + a, 1 + b)),
    //             (Some(a), None) => Some(a + 3),
    //             (None, Some(b)) => Some(b + 1),
    //             (None, None) => None,
    //         },
    //     );
    //     dp[&curr_sum]
    // }
}

pub fn solve(input: String) -> Result<String> {
    let output = input
        .split("\n\n")
        .map(Game::new)
        .flat_map(|game| game.play())
        .sum::<f64>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let expected = "480";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
