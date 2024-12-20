use std::collections::{HashSet, VecDeque};

use super::Result;
// find regions
// for all regions find area and perimeter
// multiply area with perimeter
// will take each region as separate
pub fn solve(input: String) -> Result<String> {
    let mut matrix = load_data(&input);
    let n = matrix.len();
    if n == 0 || matrix[0].len() != n {
        return Err("Invalid input. Check the size of Map.".into());
    }
    let mut count = 0;
    for row in 0..n {
        for col in 0..n {
            if matrix[row][col] == b'.' {
                continue;
            }
            count += bfs(&mut matrix, n, (row, col));
        }
    }
    Ok(count.to_string())
}

fn load_data(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect()
}

fn neighbor(n: usize, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let mut nbrs = Vec::with_capacity(4);
    // up
    if row > 0 {
        nbrs.push((row - 1, col));
    }
    // down
    if row + 1 < n {
        nbrs.push((row + 1, col));
    }
    // left
    if col > 0 {
        nbrs.push((row, col - 1));
    }
    // right
    if col + 1 < n {
        nbrs.push((row, col + 1));
    }
    nbrs.into_iter()
}

fn bfs(matrix: &mut [Vec<u8>], n: usize, (row, col): (usize, usize)) -> usize {
    let mut cells = HashSet::new();
    let mut queue = VecDeque::new();
    let mut perimeter = 0;
    queue.push_back((row, col));
    let value = matrix[row][col];
    // println!("{}", value as char);
    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            if let Some((r, c)) = queue.pop_front() {
                if matrix[r][c] == b'.' {
                    continue;
                }
                matrix[r][c] = b'.';
                let inserted = cells.insert((r, c));
                if inserted {
                    perimeter += 4;
                }
                for (r_next, c_next) in neighbor(n, (r, c)) {
                    if inserted && cells.contains(&(r_next, c_next)) {
                        perimeter -= 2;
                    }
                    if matrix[r_next][c_next] == b'.' || value != matrix[r_next][c_next] {
                        continue;
                    }
                    queue.push_back((r_next, c_next));
                }
            }
        }
        // println!("{}, ", queue.len());
    }
    let area = cells.len();
    // println!("\n\tarea: {area}\n\tperimeter: {perimeter}");
    area * perimeter
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_1() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let expected = "140";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let expected = "772";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_3() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let expected = "1930";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
