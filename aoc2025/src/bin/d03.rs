use aoc2025::run_day;

use anyhow::Result;
use iter_first_max::IterFirstMaxExt;

const DAY: &str = "d03";

fn ex1(file: &str) -> Result<i64> {
    let s = file
        .lines()
        .filter_map(|l| {
            let len = l.len();
            let batteries_iter = l.chars().filter_map(|c| c.to_digit(10));
            let max = batteries_iter
                .clone()
                .take(len - 1)
                .enumerate()
                .first_max_by_key(|(_idx, val)| *val)?;
            let second_max = batteries_iter.skip(max.0 + 1).max()?;
            format!("{}{}", max.1, second_max).parse::<i64>().ok()
        })
        .sum::<i64>();
    Ok(s)
}

fn ex2(file: &str) -> Result<i64> {
    let s = file
        .lines()
        .filter_map(|l| {
            let len = l.len();
            let batteries_iter = l.chars().filter_map(|c| c.to_digit(10));
            let mut max_ind = -1isize;
            let mut chars: [u8; 12] = [0; 12];
            for ind in 1..=12 {
                let max = batteries_iter
                    .clone()
                    .enumerate()
                    .take(len - (12 - ind))
                    .skip((max_ind + 1) as usize)
                    .first_max_by_key(|(_idx, val)| *val)?;
                max_ind = max.0 as isize;
                chars[ind - 1] = char::from_digit(max.1, 10)? as u8;
            }
            str::from_utf8(&chars).ok()?.parse::<i64>().ok()
        })
        .sum::<i64>();
    Ok(s)
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
        let example = "987654321111111
811111111111119
234234234234278
818181911112111
";
        let expected_ex1: i64 = 357;
        let expected_ex2: i64 = 3121910778619;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 17405;
        let expected_ex2: i64 = 171990312704598;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
