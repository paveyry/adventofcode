use aoc2025::run_day;

use anyhow::{Error, Result};

const DAY: &str = "d00";

fn ex1(file: &str) -> Result<i64> {
    Ok(0)
}

fn ex2(file: &str) -> Result<i64> {
    Ok(0)
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
        let example = "";
        let expected_ex1: i64 = 0;
        let expected_ex2: i64 = 0;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        // assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    // #[test]
    // fn test_file() {
    //     let file =
    //         fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
    //     let expected_ex1: i64 = 0;
    //     let expected_ex2: i64 = 0;
    //     assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
    //     // assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    // }
}
