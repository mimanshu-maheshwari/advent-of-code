use std::collections::{HashMap, HashSet};

use super::Result;
pub fn solve(input: String) -> Result<String> {
    let mut matrix = generate_matrix(input);
    let mut loc_map: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let (rows, cols) = (matrix.len(), matrix[0].len());
    for (r, row) in matrix.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            if col == b'.' {
                continue;
            }
            loc_map
                .entry(col)
                .and_modify(|locs| locs.push((r, c)))
                .or_insert(vec![(r, c)]);
        }
    }
    let mut result_set = HashSet::new();
    for (_, locations) in loc_map {
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let (r1, c1) = locations[i];
                let (r2, c2) = locations[j];
                let (r1, c1) = (r1 as isize, c1 as isize);
                let (r2, c2) = (r2 as isize, c2 as isize);
                result_set.insert((r1, c1));
                result_set.insert((r2, c2));
                // find difference
                let (rdif, cdif) = (r2 - r1, c2 - c1);
                // find the multiplier 2
                // add diff to 2,
                {
                    let (mut r_new, mut c_new) = (r2 + rdif, c2 + cdif);
                    while r_new >= 0 && r_new < rows as isize && c_new >= 0 && c_new < cols as isize
                    {
                        result_set.insert((r_new, c_new));
                        if matrix[r_new as usize][c_new as usize] == b'.' {
                            matrix[r_new as usize][c_new as usize] = b'#';
                        }
                        (r_new, c_new) = (r_new + rdif, c_new + cdif);
                    }
                }
                // find the negation 1
                // sub  diff from 1
                {
                    let (mut r_new, mut c_new) = (r1 - rdif, c1 - cdif);
                    while r_new >= 0 && r_new < rows as isize && c_new >= 0 && c_new < cols as isize
                    {
                        result_set.insert((r_new, c_new));
                        if matrix[r_new as usize][c_new as usize] == b'.' {
                            matrix[r_new as usize][c_new as usize] = b'#';
                        }
                        (r_new, c_new) = (r_new - rdif, c_new - cdif);
                    }
                }
            }
        }
    }
    // print_mat(&matrix);
    Ok(result_set.len().to_string())
}

#[allow(dead_code)]
fn print_mat(matrix: &[Vec<u8>]) -> String {
    let mut res = String::new();
    for row in matrix {
        let row = row.iter().map(|&v| v as char).collect::<Vec<_>>();
        for v in row {
            res.push(v);
            print!("{v}");
        }
        res.push('\n');
    }
    res
}

fn generate_matrix(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.bytes().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example_01() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let expected = "34";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
    #[test]
    fn example_02() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let expected = "9";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}

/*
##....#....#  ##....#....#
.#.#....0...  .#.#....0...
..#.#0....#.  ..#.#0....#.
..##...0....  ..##...0....
....0....#..  ....0....#..
.#...#A....#  .#...#A....#
...#..#.....  ...#..#.....
#....#.#....  #....#.#....
..#.....A...  ..#.....A...
....#....A..  ....#....A..
.#........#.  .#........#.
...#......##  ...#......##

*/
