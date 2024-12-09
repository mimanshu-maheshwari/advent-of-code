use std::collections::HashMap;

use super::Result;

pub fn solve(input: String) -> Result<String> {
    let mut blanks = Vec::new();
    let mut files = HashMap::new();
    let (mut fid, mut pos) = (0, 0);
    let _: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|line| line.bytes().map(|b| (b - b'0') as usize))
        .enumerate()
        .map(|(index, val)| {
            if index & 1 == 0 {
                files.insert(fid, (pos, val));
                fid += 1;
            } else if val != 0 {
                blanks.push((pos, val));
            }
            pos += val;
        })
        .collect();
    while fid > 0 {
        fid -= 1;
        let (pos, size) = files[&fid];
        for (i, &(start, length)) in blanks.iter().enumerate() {
            if start >= pos {
                blanks = blanks[..i].to_vec();
                break;
            }
            if size <= length {
                files.insert(fid, (start, size));
                if size == length {
                    blanks.remove(i);
                } else {
                    blanks[i] = (start + size, length - size);
                }
                break;
            }
        }
    }
    Ok(files
        .into_iter()
        .map(|(fid, (pos, size))| (pos..(pos + size)).map(|num| num * fid).sum::<usize>())
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "2333133121414131402";
        let expected = "2858";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
