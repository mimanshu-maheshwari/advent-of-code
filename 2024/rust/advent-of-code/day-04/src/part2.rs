use super::Result;

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Direction {
    fn neighbor(&self, rows: usize, cols: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
        use Direction::*;
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
    let search_matrix1 = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    let search_matrix2 = vec![
        vec!['S', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'M'],
    ];
    let search_matrix3 = vec![
        vec!['M', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'S'],
    ];
    let search_matrix4 = vec![
        vec!['S', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'M'],
    ];
    let matrix = create_matrix(input);
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut count = 0;
    for row in 0..(rows - 2) {
        for col in 0..(cols - 2) {
            if check_xmas(&matrix, (row, col), &search_matrix1)
                || check_xmas(&matrix, (row, col), &search_matrix2)
                || check_xmas(&matrix, (row, col), &search_matrix3)
                || check_xmas(&matrix, (row, col), &search_matrix4)
            {
                count += 1;
            }
        }
    }
    Ok(count.to_string())
}

fn check_xmas(
    matrix: &[Vec<char>],
    (row, col): (usize, usize),
    search_matrix: &[Vec<char>],
) -> bool {
    for r in 0..3 {
        for c in 0..3 {
            if search_matrix[r][c] == '.' {
                continue;
            }
            // println!("row: {}, col: {}, r: {}, c: {}", row, col, r, c);
            if matrix[row + r][col + c] != search_matrix[r][c] {
                return false;
            }
        }
    }
    true
}

fn create_matrix(input: String) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }
        matrix.push(line.chars().collect::<Vec<_>>());
    }
    // println!("{:?}", matrix);
    matrix
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_2() {
        let input = ".M.S......

..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let expected = "9";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_1() {
        let input = "M.S
.A.

M.S";
        let expected = "1";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
