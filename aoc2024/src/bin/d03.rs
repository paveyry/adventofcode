use aoc2024::run_day;

use anyhow::Result;
use regex::Regex;

const DAY: &str = "d03";

const DO: &str = "do";
const DONT: &str = "don't";

fn ex1(file: &str) -> Result<i64> {
    Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?
        .captures_iter(file)
        .map(|c| c.extract())
        .map(|(_, [arg1, arg2])| Ok(arg1.parse::<i64>()? * arg2.parse::<i64>()?))
        .sum()
}

fn ex2(file: &str) -> Result<i64> {
    Regex::new(r"(?:mul\(([0-9]+),([0-9]+)\))|(?:(do)\(()\))|(?:(don't)\(()\))")?
        .captures_iter(file)
        .map(|c| c.extract())
        .fold(Ok((0, true)), |prev, cap| {
            let (prev_val, do_action) = prev?;
            match cap.1 {
                [DO, _] => Ok((prev_val, true)),
                [DONT, _] => Ok((prev_val, false)),
                [arg1, arg2] => Ok((
                    (if do_action {
                        prev_val + arg1.parse::<i64>()? * arg2.parse::<i64>()?
                    } else {
                        prev_val
                    }),
                    do_action,
                )),
            }
        })
        .map(|tup| tup.0)
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
        let example1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_ex1: i64 = 161;
        let expected_ex2: i64 = 48;
        assert_eq!(expected_ex1, ex1(example1).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example2).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 163931492;
        let expected_ex2: i64 = 76911921;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
