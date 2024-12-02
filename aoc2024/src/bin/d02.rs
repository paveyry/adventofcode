use std::cmp::Ordering;

use aoc2024::run_day;

use anyhow::{Context, Result};
use itertools::Itertools;

const DAY: &str = "d02";

fn ex1(file: &str) -> Result<i64> {
    let mut num_safe = 0;

    let filtered_iter = file
        .lines()
        .map(|l| -> Result<bool> {
            let mut split = l.split_ascii_whitespace();
            let mut prev_num = split
                .next()
                .context("missing first element in split")?
                .parse::<i64>()?;
            let mut prev_ord = Ordering::Equal;
            for e in split {
                let n = e.parse::<i64>()?;
                let ord = n.cmp(&prev_num);
                if !matches!(prev_ord, Ordering::Equal) && ord != prev_ord
                    || !matches!((n - prev_num).abs(), 1..=3)
                {
                    return Ok(false);
                }
                prev_ord = ord;
                prev_num = n;
            }
            Ok(true)
        })
        .filter_ok(|e| *e);
    for r in filtered_iter {
        r?;
        num_safe += 1;
    }

    Ok(num_safe)
}

fn ex2(file: &str) -> Result<i64> {
    let mut num_safe = 0;
    for l in file.lines() {
        let levels = l
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().map_err(Into::into))
            .collect::<Result<Vec<_>>>()?;
        // this approach is suboptimal because it would probably be possible to detect errors
        // while checking the line and just skip that one element but the first number case is
        // annoying because either the first or second element could be skipped so I did not bother.
        // This still runs in just a few microseconds.
        for skip_elt in (0..=levels.len()).rev() {
            // first iteration is outside of range (levels.len()) so it won't skip anything
            let mut iter =
                levels
                    .iter()
                    .enumerate()
                    .filter_map(|(i, n)| if i != skip_elt { Some(n) } else { None });
            let mut prev_num = *iter.next().context("missing first element in split")?;
            let mut prev_ord = Ordering::Equal;
            let mut result = true;
            for n in iter {
                let ord = n.cmp(&prev_num);
                if !matches!(prev_ord, Ordering::Equal) && ord != prev_ord
                    || !matches!((n - prev_num).abs(), 1..=3)
                {
                    result = false;
                    break;
                }
                prev_ord = ord;
                prev_num = *n;
            }
            if result {
                num_safe += 1;
                break;
            }
        }
    }

    Ok(num_safe)
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
        let example = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let expected_ex1: i64 = 2;
        let expected_ex2: i64 = 4; // FIXME (output from example)
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 371;
        let expected_ex2: i64 = 426; // FIXME (output computed when ex1 passed check)
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
