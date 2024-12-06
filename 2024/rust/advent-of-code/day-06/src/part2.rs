use std::{collections::HashSet, thread};

use super::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    #[inline(always)]
    fn change(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    #[inline(always)]
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

#[inline(always)]
pub fn solve(input: String) -> Result<String> {
    let input = gen_matrix(input);
    let (rows, cols) = (input.len(), input[0].len());
    let (mut r, mut c) = (rows + 1, cols + 1);
    'row_loop: for (row, row_val) in input.iter().enumerate() {
        '_col_loop: for (col, col_val) in row_val.iter().enumerate() {
            if [b'^', b'>', b'<', b'v'].contains(col_val) {
                (r, c) = (row, col);
                break 'row_loop;
            } else {
                (r, c) = (rows + 1, cols + 1);
            }
        }
    }
    if (r, c) == (rows + 1, cols + 1) {
        return Err("Unable to find the guards position".into());
    }
    let mut handlers = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if [b'#', b'^', b'>', b'<', b'v'].contains(&input[row][col]) {
                continue;
            }
            let mut matrix = input.clone();
            let handler = thread::spawn(move || {
                matrix[row][col] = b'#';
                validate(&mut matrix, (rows, cols), (r, c))
            });

            handlers.push(handler);
            // println!("[{row}, {col}]");
            // handlers.push(handler.join().unwrap_or(false));
        }
    }
    let result = handlers
        .into_iter()
        .flat_map(|handler| handler.join())
        .map(|val| if val { 1 } else { 0 })
        .sum::<usize>();
    Ok(result.to_string())
}

#[inline(always)]
fn validate(
    matrix: &mut [Vec<u8>],
    (rows, cols): (usize, usize),
    (row, col): (usize, usize),
) -> bool {
    if matrix[row][col] == b'^' {
        mark_x(matrix, (rows, cols), Direction::Up, (row, col))
    } else if matrix[row][col] == b'<' {
        mark_x(matrix, (rows, cols), Direction::Left, (row, col))
    } else if matrix[row][col] == b'>' {
        mark_x(matrix, (rows, cols), Direction::Right, (row, col))
    } else if matrix[row][col] == b'v' {
        mark_x(matrix, (rows, cols), Direction::Down, (row, col))
    } else {
        false
    }
}

#[inline(always)]
fn mark_x(
    matrix: &mut [Vec<u8>],
    (rows, cols): (usize, usize),
    mut direction: Direction,
    (mut row, mut col): (usize, usize),
) -> bool {
    let mut visited = HashSet::new();
    visited.insert((row, col, direction));
    while let Some((r, c)) = direction.next(rows, cols, row, col) {
        if visited.contains(&(r, c, direction)) {
            return true;
        }
        if matrix[r][c] == b'#' {
            direction = direction.change();
            continue;
        }
        visited.insert((r, c, direction));
        (row, col) = (r, c);
    }
    false
}

#[inline(always)]
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
        let expected = "6";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}

/*
....#.....  ....#.....  ....#.....  ....#.....  ....#.....  ....#.....  ....#.....
.........#  ....+---+#  ....+---+#  ....+---+#  ....+---+#  ....+---+#  ....+---+#
..........  ....|...|.  ....|...|.  ....|...|.  ....|...|.  ....|...|.  ....|...|.
..#.......  ..#.|...|.  ..#.|...|.  ..#.|...|.  ..#.|...|.  ..#.|...|.  ..#.|...|.
.......#..  ....|..#|.  ..+-+-+#|.  ..+-+-+#|.  ..+-+-+#|.  ..+-+-+#|.  ..+-+-+#|.
..........  ....|...|.  ..|.|.|.|.  ..|.|.|.|.  ..|.|.|.|.  ..|.|.|.|.  ..|.|.|.|.
.#..^.....  .#.O^---+.  .#+-^-+-+.  .#+-^-+-+.  .#+-^-+-+.  .#+-^-+-+.  .#+-^-+-+.
........#.  ........#.  ......O.#.  .+----+O#.  ..|...|.#.  ....|.|.#.  .+----++#.
#.........  #.........  #.........  #+----+...  #O+---+...  #..O+-+...  #+----++..
......#...  ......#...  ......#...  ......#...  ......#...  ......#...  ......#O..
*/
