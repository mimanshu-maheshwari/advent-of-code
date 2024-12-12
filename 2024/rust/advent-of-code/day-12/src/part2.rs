use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::{Add, Deref, Div, Sub},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct F64(f64);
impl Deref for F64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Div<F64> for F64 {
    type Output = F64;
    fn div(self, rhs: F64) -> Self::Output {
        F64(self.0 / rhs.0)
    }
}

impl Div<f64> for F64 {
    type Output = F64;
    fn div(self, rhs: f64) -> Self::Output {
        F64(self.0 / rhs)
    }
}

impl Add<F64> for F64 {
    type Output = F64;
    fn add(self, rhs: F64) -> Self::Output {
        F64(self.0 + rhs.0)
    }
}

impl Add<f64> for F64 {
    type Output = F64;
    fn add(self, rhs: f64) -> Self::Output {
        F64(self.0 + rhs)
    }
}

impl Sub<F64> for F64 {
    type Output = F64;
    fn sub(self, rhs: F64) -> Self::Output {
        F64(self.0 - rhs.0)
    }
}

impl Sub<f64> for F64 {
    type Output = F64;
    fn sub(self, rhs: f64) -> Self::Output {
        F64(self.0 - rhs)
    }
}

impl Eq for F64 {}

impl Hash for F64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_bits().to_ne_bytes());
    }
}

impl From<usize> for F64 {
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

use super::Result;
// find regions
// for all regions find area and perimeter
// multiply area with perimeter
// will take each region as separate
pub fn solve(input: String) -> Result<String> {
    let matrix = load_data(&input);
    let n = matrix.len();
    if n == 0 || matrix[0].len() != n {
        return Err("Invalid input. Check the size of Map.".into());
    }
    let mut count = 0;
    let mut visited = HashSet::new();
    for row in 0..n {
        for col in 0..n {
            if visited.contains(&(row, col)) {
                continue;
            }
            count += bfs(&matrix, n, &mut visited, (row, col));
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

fn _diagonals(n: usize, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let mut nbrs = Vec::with_capacity(4);
    // up left
    if row > 0 && col > 0 {
        nbrs.push((row - 1, col - 1));
    }
    // up right
    if row > 0 && col + 1 < n {
        nbrs.push((row - 1, col + 1));
    }
    // down left
    if row + 1 < n && col > 0 {
        nbrs.push((row + 1, col - 1));
    }
    // down right
    if row + 1 < n && col + 1 < n {
        nbrs.push((row + 1, col + 1));
    }
    nbrs.into_iter()
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

fn bfs(
    matrix: &[Vec<u8>],
    n: usize,
    visited: &mut HashSet<(usize, usize)>,
    (row, col): (usize, usize),
) -> usize {
    let mut cells = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((row, col));
    let value = matrix[row][col];
    println!("{}", value as char);
    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            if let Some((r, c)) = queue.pop_front() {
                if visited.contains(&(r, c)) {
                    continue;
                }
                visited.insert((r, c));
                let _inserted = cells.insert((r, c));

                for (r_next, c_next) in neighbor(n, (r, c)) {
                    if visited.contains(&(r_next, c_next)) || value != matrix[r_next][c_next] {
                        continue;
                    }
                    queue.push_back((r_next, c_next));
                }
            }
        }
        // println!("{}, ", queue.len());
    }
    let area = cells.len();
    let sides = calc_sides(&cells);
    println!("\n\tarea: {area}\n\tsides: {sides}");
    area * sides
}

fn calc_sides(region: &HashSet<(usize, usize)>) -> usize {
    let mut corner_candidate = HashSet::new();
    let region = region
        .iter()
        .map(|&(r, c)| (F64(r as f64), F64(c as f64)))
        .collect::<HashSet<_>>();
    // collecting corners
    for &(r, c) in &region {
        for (cr, cc) in [
            (r - 0.5, c - 0.5),
            (r + 0.5, c - 0.5),
            (r + 0.5, c + 0.5),
            (r - 0.5, c + 0.5),
        ] {
            corner_candidate.insert((cr, cc));
        }
    }
    let mut corners = 0;
    for (cr, cc) in corner_candidate {
        let config = [
            (cr - 0.5, cc - 0.5),
            (cr + 0.5, cc - 0.5),
            (cr + 0.5, cc + 0.5),
            (cr - 0.5, cc + 0.5),
        ]
        .into_iter()
        .map(|v| region.contains(&v))
        .collect::<Vec<_>>();
        let number = config.iter().map(|v| if *v { 1 } else { 0 }).sum::<usize>();
        if number == 1 {
            corners += 1;
        } else if number == 2 {
            if config == vec![true, false, true, false] || config == vec![false, true, false, true]
            {
                corners += 2;
            }
        } else if number == 3 {
            corners += 1;
        }
    }
    corners
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
        let expected = "80";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let expected = "236";
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
        let expected = "1206";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
