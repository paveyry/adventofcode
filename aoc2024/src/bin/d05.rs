use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use aoc2024::run_day;

use anyhow::{Context, Result};
use itertools::Itertools;

const DAY: &str = "d05";

struct RuleChecker(HashMap<i64, (HashSet<i64>, HashSet<i64>)>);

impl RuleChecker {
    fn new() -> Self {
        RuleChecker(HashMap::new())
    }

    fn compare(&self, a: i64, b: i64) -> Ordering {
        match self.0.get(&a) {
            Some((less_vals, _)) if less_vals.contains(&b) => Ordering::Greater,
            Some((_, greater_vals)) if greater_vals.contains(&b) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

struct Update {
    middle_value: i64,
    values: Vec<i64>,
    page_pos: HashMap<i64, usize>,
}

fn parse(file: &str) -> Result<(RuleChecker, Vec<Update>)> {
    let mut sections_iter = file.split("\n\n");

    let mut checker = RuleChecker::new();
    sections_iter
        .next()
        .context("missing rules section")?
        .lines()
        .map(|l| {
            l.split('|')
                .next_tuple()
                .context("invalid rule line")
                .map(|(a, b)| Ok::<_, anyhow::Error>((a.parse::<i64>()?, b.parse::<i64>()?)))?
        })
        .try_for_each(|r| {
            let (low, high) = r?;
            checker.0.entry(low).or_default().1.insert(high);
            checker.0.entry(high).or_default().0.insert(low);
            Ok::<(), anyhow::Error>(())
        })?;

    let updates = sections_iter
        .next()
        .context("missing updates section")?
        .lines()
        .map(|l| {
            l.split(',')
                .map(&str::parse::<i64>)
                .enumerate()
                .map(|(pos, val)| {
                    let val = val?;
                    Ok(((val), (val, pos)))
                })
                .collect::<Result<(Vec<_>, HashMap<_, _>)>>()
        })
        .map(|r| {
            let (v, hm) = r?;
            Ok(Update {
                middle_value: *(v.get(v.len() / 2).context("failed to get middle value")?),
                values: v,
                page_pos: hm,
            })
        })
        .collect::<Result<Vec<Update>>>()?;
    Ok((checker, updates))
}

fn validate_update(upd: &Update, checker: &RuleChecker) -> i64 {
    for (pos, page) in upd.values.iter().enumerate() {
        if let Some((less_values, greater_values)) = checker.0.get(page) {
            for less_val in less_values {
                if let Some(other_pos) = upd.page_pos.get(less_val) {
                    if *other_pos > pos {
                        return 0;
                    }
                }
            }
            for greater_val in greater_values {
                if let Some(other_pos) = upd.page_pos.get(greater_val) {
                    if *other_pos < pos {
                        return 0;
                    }
                }
            }
        }
    }
    upd.middle_value
}

fn ex1(file: &str) -> Result<i64> {
    let (checker, updates) = parse(file)?;

    Ok(updates
        .into_iter()
        .map(|upd| validate_update(&upd, &checker))
        .sum())
}

fn ex2(file: &str) -> Result<i64> {
    let (checker, updates) = parse(file)?;

    Ok(updates
        .into_iter()
        .filter(|upd| validate_update(upd, &checker) == 0)
        .map(|mut upd| {
            upd.values.sort_by(|a, b| checker.compare(*a, *b));
            upd.values[upd.values.len() / 2]
        })
        .sum())
}

fn main() {
    run_day(DAY, ex1, ex2);
}

#[cfg(test)]
mod tests {
    use super::DAY;
    use std::fs;

    use super::{ex1, ex2};

    #[test]
    fn test() {
        let example = "47|53
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
        let expected_ex1: i64 = 143;
        let expected_ex2: i64 = 123;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 5588;
        let expected_ex2: i64 = 5331;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
