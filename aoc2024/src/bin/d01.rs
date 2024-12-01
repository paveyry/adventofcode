use std::collections::HashMap;

use aoc2024::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d01";

fn get_lists(file: &str) -> Result<(Vec<i64>, Vec<i64>)> {
    file.lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            (
                split
                    .next()
                    .and_then(|s| s.parse::<i64>().ok())
                    .context("split failed to get first element"),
                split
                    .next()
                    .and_then(|s| s.parse::<i64>().ok())
                    .context("split failed to get first element"),
            )
        })
        .try_fold((vec![], vec![]), |(mut prev1, mut prev2), (cur1, cur2)| {
            prev1.push(cur1?);
            prev2.push(cur2?);
            Ok((prev1, prev2))
        })
}

fn ex1(file: &str) -> Result<i64> {
    let (mut location_ids_1, mut location_ids_2) = get_lists(file)?;
    location_ids_1.sort();
    location_ids_2.sort();
    let result = location_ids_1
        .iter()
        .zip(location_ids_2)
        .map(|v| (v.1 - v.0).abs())
        .sum();
    Ok(result)
}

fn ex2(file: &str) -> Result<i64> {
    let (locations_ids_1, locations_ids_2) = get_lists(file)?;
    let mut occurences: HashMap<i64, i64> = HashMap::new();
    locations_ids_2.iter().for_each(|e| {
        *occurences.entry(*e).or_default() += 1;
    });
    let sum = locations_ids_1
        .iter()
        .map(|e| occurences.get(e).map(|v| e * *v).unwrap_or(0))
        .sum();
    Ok(sum)
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
        let example = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let expected_ex1: i64 = 11;
        let expected_ex2: i64 = 31;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 765748;
        let expected_ex2: i64 = 27732508;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
