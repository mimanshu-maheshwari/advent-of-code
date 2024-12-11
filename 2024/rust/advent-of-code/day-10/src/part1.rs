use super::Result;
use std::collections::HashSet;
pub fn solve(input: String) -> Result<String> {
    let matrix = gen_matrix(&input);
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut paths_count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 0 {
                // println!("start: [{row}, {col}]");
                let mut set = HashSet::new();
                dfs(&matrix, (rows, cols), (row, col), 0, &mut set);
                paths_count += set.len();
            }
        }
    }
    Ok(paths_count.to_string())
}

fn gen_matrix(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn dfs(
    matrix: &[Vec<u8>],
    shape: (usize, usize),
    (row, col): (usize, usize),
    current: u8,
    set: &mut HashSet<(usize, usize)>,
) {
    // base cases
    if matrix[row][col] != current {
        return;
    }

    if matrix[row][col] == 9 && current == 9 {
        // println!("\tend : [{row}, {col}]");
        set.insert((row, col));
        return;
    }

    // recursive cases
    for (r, c) in neighbors(shape, (row, col)) {
        dfs(matrix, shape, (r, c), current + 1, set);
    }
}

fn neighbors(
    (rows, cols): (usize, usize),
    (row, col): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let mut neighbors = Vec::new();
    // up
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    // down
    if row + 1 < rows {
        neighbors.push((row + 1, col));
    }
    // right
    if col + 1 < cols {
        neighbors.push((row, col + 1));
    }
    // left
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    neighbors.into_iter()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let expected = "36";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2() {
        let input = "0123
1234
8765
9876";
        let expected = "1";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}