use super::Result;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn change(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn next(&self, rows: usize, cols: usize, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row + 1 < rows {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col + 1 < cols {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
        }
    }
}

pub fn solve(input: String) -> Result<String> {
    let mut input = gen_matrix(input);
    let (rows, cols) = (input.len(), input[0].len());
    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == b'^' {
                result = mark_x(&mut input, (rows, cols), Direction::Up, (row, col));
                break;
            } else if input[row][col] == b'<' {
                result = mark_x(&mut input, (rows, cols), Direction::Left, (row, col));
                break;
            } else if input[row][col] == b'>' {
                result = mark_x(&mut input, (rows, cols), Direction::Right, (row, col));
                break;
            } else if input[row][col] == b'v' {
                result = mark_x(&mut input, (rows, cols), Direction::Down, (row, col));
                break;
            } else {
                continue;
            }
        }
    }
    Ok(result.to_string())
}

fn mark_x(
    matrix: &mut [Vec<u8>],
    (rows, cols): (usize, usize),
    mut direction: Direction,
    (mut row, mut col): (usize, usize),
) -> usize {
    let mut result = 1;
    while let Some((r, c)) = direction.next(rows, cols, row, col) {
        if matrix[r][c] == b'#' {
            direction = direction.change();
            continue;
        }
        if matrix[row][col] != b'X' {
            matrix[row][col] = b'X';
            result += 1;
        }
        (row, col) = (r, c);
    }
    result
}

fn gen_matrix(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......

.......#..
..........
.#..^.....
........#.

#.........
......#...";
        let expected = "41";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
/*
/  ....#.....  |  ....#.....  |  ....#.....  |  ....#.....  |  ....#.....  |  ....#.....  /
/  .........#  |  ....^....#  |  ........>#  |  .........#  |  .........#  |  ....XXXXX#  /
/  ..........  |  ..........  |  ..........  |  ..........  |  ..........  |  ....X...X.  /
/  ..#.......  |  ..#.......  |  ..#.......  |  ..#.......  |  ..#.......  |  ..#.X...X.  /
/  .......#..  |  .......#..  |  .......#..  |  .......#..  |  .......#..  |  ..XXXXX#X.  /
/  ..........  |  ..........  |  ..........  |  ..........  |  ..........  |  ..X.X.X.X.  /
/  .#..^.....  |  .#........  |  .#........  |  .#......v.  |  .#........  |  .#XXXXXXX.  /
/  ........#.  |  ........#.  |  ........#.  |  ........#.  |  ........#.  |  .XXXXXXX#.  /
/  #.........  |  #.........  |  #.........  |  #.........  |  #.........  |  #XXXXXXX..  /
/  ......#...  |  ......#...  |  ......#...  |  ......#...  |  ......#v..  |  ......#X..  /
*/
