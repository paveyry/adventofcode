use aoc2025::run_day;

use anyhow::{Context, Result};
use bounded_integer::BoundedI32;

const DAY: &str = "d01";

fn ex1(file: &str) -> Result<i64> {
    let mut dial = <BoundedI32<0, 99>>::new(50).context("failed to init bounded int")?;
    let mut count_zeroes = 0;
    for l in file.lines() {
        let rot_num: i32 = l[1..].parse()?;
        if let Some('L') = l.chars().next() {
            dial = dial.wrapping_sub(rot_num);
        } else {
            dial = dial.wrapping_add(rot_num);
        }
        if dial == 0 {
            count_zeroes += 1;
        }
    }
    Ok(count_zeroes)
}

fn ex2(file: &str) -> Result<i64> {
    let mut dial = <BoundedI32<0, 99>>::new(50).context("failed to init bounded int")?;
    let mut count_zeroes: i64 = 0;
    for l in file.lines() {
        let rot_num: i32 = l[1..].parse()?;
        if let Some('L') = l.chars().next() {
            let sub = dial.get() - rot_num;
            let base = if dial == 0 { 0 } else { 1 };
            if sub <= 0 {
                count_zeroes += (base + sub.abs() / 100) as i64;
            }
            dial = dial.wrapping_sub(rot_num);
        } else {
            let add = dial.get() + rot_num;
            if add > 99 {
                count_zeroes += (add / 100) as i64;
            }
            dial = dial.wrapping_add(rot_num);
        }
    }
    Ok(count_zeroes)
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
        let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected_ex1: i64 = 3; // FIXME (output from example)
        let expected_ex2: i64 = 6; // FIXME (output from example)
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    // #[test]
    // fn test_file() {
    //     let file =
    //         fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
    //     let expected_ex1: i64 = 0; // FIXME (output computed when ex1 passed check)
    //     let expected_ex2: i64 = 0; // FIXME (output computed when ex1 passed check)
    //     assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
    //     assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    // }
}
