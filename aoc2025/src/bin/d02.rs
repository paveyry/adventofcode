use aoc2025::run_day;

use anyhow::Result;

const DAY: &str = "d02";

fn ex1(file: &str) -> Result<i64> {
    Ok(file
        .trim_ascii()
        .split(',')
        .filter_map(|rang| rang.split_once('-'))
        .filter_map(|(low, high)| Some((low.parse::<i64>().ok()?, high.parse::<i64>().ok()?)))
        .map(|(low, high)| {
            (low..=high)
                .filter(|&id| {
                    let s = id.to_string();
                    s[..s.len() / 2] == s[s.len() / 2..]
                })
                .sum::<i64>()
        })
        .sum())
}

fn ex2(file: &str) -> Result<i64> {
    Ok(file
        .trim_ascii()
        .split(',')
        .filter_map(|rang| rang.split_once('-'))
        .filter_map(|(low, high)| Some((low.parse::<i64>().ok()?, high.parse::<i64>().ok()?)))
        .map(|(low, high)| {
            (low..=high)
                .filter(|&id| {
                    let s = id.to_string();
                    for slicelen in (1..=s.len() / 2).filter(|len| s.len() % len == 0) {
                        let mut ok = true;
                        for k in 0..s.len() / slicelen - 1 {
                            if s[k * slicelen..(k + 1) * slicelen]
                                != s[(k + 1) * slicelen..(k + 2) * slicelen]
                            {
                                ok = false;
                            }
                        }
                        if ok {
                            return true;
                        }
                    }
                    false
                })
                .sum::<i64>()
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
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected_ex1: i64 = 1227775554;
        let expected_ex2: i64 = 4174379265;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 9188031749;
        let expected_ex2: i64 = 11323661261;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
