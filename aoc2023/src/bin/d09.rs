use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

use anyhow::{Error, Result};

fn sum_extrapolations(file: &str, reverse: bool) -> Result<i64> {
    let mut sum = 0;
    for l in file.lines() {
        let mut numbers = l
            .split_whitespace()
            .map(str::parse::<i64>)
            .collect::<std::result::Result<Vec<i64>, ParseIntError>>()?;
        if reverse {
            numbers.reverse();
        }
        let last = *numbers.last().ok_or_else(|| Error::msg("vec is empty"))?;
        let mut all_zero = false;
        let mut sum_last_elements = 0;
        while !all_zero {
            all_zero = true;
            let mut diff = 0;
            for i in 0..numbers.len() - 1 {
                diff = numbers[i + 1] - numbers[i];
                if diff != 0 {
                    all_zero = false;
                }
                numbers[i] = diff;
            }
            numbers.remove(numbers.len() - 1);
            sum_last_elements += diff;
        }
        sum += last + sum_last_elements;
    }
    Ok(sum)
}

fn ex1(file: &str) -> Result<i64> {
    sum_extrapolations(file, false)
}

fn ex2(file: &str) -> Result<i64> {
    sum_extrapolations(file, true)
}

fn main() {
    let file = fs::read_to_string("./inputs/d09_1.txt").unwrap();

    let start = Instant::now();
    let res_ex1 = ex1(&file);
    let duration = start.elapsed();
    println!("ex1: {} (computed in {:?})", res_ex1.unwrap(), duration);

    let start = Instant::now();
    let res_ex2 = ex2(&file);
    let duration = start.elapsed();
    println!("ex2: {} (computed in {:?})", res_ex2.unwrap(), duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, ex1(input).unwrap());
        assert_eq!(2, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d09_1.txt").unwrap();
        assert_eq!(1806615041, ex1(&file).unwrap());
        assert_eq!(1211, ex2(&file).unwrap());
    }
}
