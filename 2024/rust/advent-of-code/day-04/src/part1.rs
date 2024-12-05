use super::Result;

#[derive(Clone, Debug, Copy)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

use Direction::*;
impl Direction {
    fn neighbor(&self, rows: usize, cols: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbor = Vec::new();
        match *self {
            UpLeft => {
                if col > 0 && row > 0 {
                    neighbor.push((row - 1, col - 1));
                }
            }
            Up => {
                if row > 0 {
                    neighbor.push((row - 1, col));
                }
            }
            UpRight => {
                if row > 0 && col + 1 < cols {
                    neighbor.push((row - 1, col + 1));
                }
            }
            Left => {
                if col > 0 {
                    neighbor.push((row, col - 1));
                }
            }
            Right => {
                if col + 1 < cols {
                    neighbor.push((row, col + 1));
                }
            }
            DownLeft => {
                if row + 1 < rows && col > 0 {
                    neighbor.push((row + 1, col - 1));
                }
            }
            Down => {
                if row + 1 < rows {
                    neighbor.push((row + 1, col));
                }
            }
            DownRight => {
                if row + 1 < rows && col + 1 < cols {
                    neighbor.push((row + 1, col + 1));
                }
            }
        }
        neighbor
    }
}

pub fn solve(input: String) -> Result<String> {
    let search_str = "XMAS".chars().collect::<Vec<_>>();
    let matrix = create_matrix(input);
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut count = 0;
    let index = 0;
    for row in 0..rows {
        for col in 0..cols {
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::UpLeft,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::Up,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::UpRight,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::Left,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::Right,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::DownLeft,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::Down,
            );
            count += dfs(
                &matrix,
                (rows, cols),
                (row, col),
                &search_str,
                index,
                Direction::DownRight,
            );
        }
    }
    Ok(count.to_string())
}
fn dfs(
    matrix: &Vec<Vec<char>>,
    (rows, cols): (usize, usize),
    (row, col): (usize, usize),
    search_str: &[char],
    index: usize,
    direction: Direction,
) -> usize {
    if index >= search_str.len() || matrix[row][col] != search_str[index] {
        return 0;
    }
    if index + 1 == search_str.len() && matrix[row][col] == search_str[index] {
        return 1;
    }
    let mut count = 0;
    for v in direction.neighbor(rows, cols, row, col) {
        count += dfs(matrix, (rows, cols), v, search_str, index + 1, direction);
    }
    count
}

fn create_matrix(input: String) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }
        matrix.push(line.chars().collect::<Vec<_>>());
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "MMMSXXMASM

MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM

MXMXAXMASX";
        let expected = "18";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
