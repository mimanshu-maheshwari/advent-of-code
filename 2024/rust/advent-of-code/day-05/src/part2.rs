use std::collections::{HashMap, HashSet};

use super::Result;

pub fn solve(input: String) -> Result<String> {
    let mut rules = Rules::new();
    if let Some((rule_lines, ledger_lines)) = input.split_once("\n\n") {
        // println!("{rules}\n---\n{ledgers}");
        rule_lines.lines().for_each(|line| {
            rules.push(
                line.trim()
                    .split_once("|")
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                    .unwrap(),
            );
        });

        return Ok(ledger_lines
            .lines()
            .map(|line| {
                line.split(',')
                    .flat_map(|v| v.trim().parse::<usize>())
                    .collect::<Vec<_>>()
            })
            .filter(|line| !rules.validate(line))
            // .inspect(|line| println!("{line:?}"))
            .map(|line| rules.correct(line))
            .map(|line| line[line.len() / 2])
            .sum::<usize>()
            .to_string());
    }
    Err("Invalid input".into())
}

#[derive(Debug)]
struct Rules {
    data: HashMap<usize, HashSet<usize>>,
}

impl Rules {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn push(&mut self, (start, end): (usize, usize)) {
        if let Some(values) = self.data.get_mut(&start) {
            values.insert(end);
        } else {
            let mut set = HashSet::new();
            set.insert(end);
            self.data.insert(start, set);
        }
        // for (_, values) in self.data.iter_mut() {
        //     if values.contains(&start) {
        //         values.insert(end);
        //     }
        // }
        self.data.entry(end).or_default();
    }

    fn validate(&self, ledger: &[usize]) -> bool {
        for i in (0..ledger.len()).rev() {
            if let Some(values) = self.data.get(&ledger[i]) {
                for val in ledger.iter().skip(i + 1) {
                    if !values.contains(val) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn correct(&self, ledger: Vec<usize>) -> Vec<usize> {
        let mut new_ledger = ledger.clone();

        while !self.validate(&new_ledger) {
            let ledger = new_ledger.clone();
            for i in (0..ledger.len()).rev() {
                if let Some(values) = self.data.get(&ledger[i]) {
                    for (j, after) in ledger.iter().enumerate().skip(i + 1) {
                        if !values.contains(after) {
                            let val = new_ledger.remove(j);
                            new_ledger.insert(i, val);
                            break;
                        }
                    }
                }
            }
        }
        new_ledger
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let expected = "123";
        let actual = solve(input.to_owned()).expect("Failed to solve input.");
        assert_eq!(expected, actual);
    }
}
