use aoc2025::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d06";

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
}

fn ex1(file: &str) -> Result<i64> {
    let mut iter = file.lines().rev();
    let ops = iter
        .next()
        .context("failed to extract ops")?
        .split_whitespace()
        .map(|s| match s {
            "*" => Ok(Operation::Multiplication),
            "+" => Ok(Operation::Addition),
            _ => Err(anyhow::Error::msg("invalid op")),
        })
        .collect::<Result<Vec<_>>>()?;
    let nums = iter
        .rev()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().map_err(Into::into))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let sum = ops
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let iter = nums.iter().map(|v| v[i]);
            match op {
                Operation::Addition => iter.sum::<i64>(),
                Operation::Multiplication => iter.product(),
            }
        })
        .sum();
    Ok(sum)
}

fn ex2(file: &str) -> Result<i64> {
    let mut iter = file.lines().rev();
    let first_line = iter.next().context("failed to extract ops")?;
    let ops = first_line
        .split_whitespace()
        .map(|s| {
            let start_index = s.as_ptr() as usize - first_line.as_ptr() as usize;
            (start_index, s)
        })
        .map(|(ind, s)| match s {
            "*" => Ok((ind, Operation::Multiplication)),
            "+" => Ok((ind, Operation::Addition)),
            _ => Err(anyhow::Error::msg("invalid op")),
        })
        .collect::<Result<Vec<_>>>()?;

    let nums_lines: Vec<&[u8]> = iter.rev().map(|l| l.as_bytes()).collect();

    let sum = ops
        .iter()
        .enumerate()
        .map(|(i, (start_index, op))| {
            let end_index = if i == ops.len() - 1 {
                nums_lines[0].len()
            } else {
                ops[i + 1].0 - 1
            };
            let nums_iter = (*start_index..end_index).filter_map(|pos| {
                let built_num = str::from_utf8(
                    nums_lines
                        .iter()
                        .filter_map(|line| {
                            if line[pos].is_ascii_digit() {
                                Some(line[pos])
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<u8>>()
                        .as_slice(),
                )
                .ok()?
                .parse::<i64>()
                .ok()?;
                Some(built_num)
            });
            match op {
                Operation::Addition => nums_iter.sum::<i64>(),
                Operation::Multiplication => nums_iter.product::<i64>(),
            }
        })
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
        let example = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        let expected_ex1: i64 = 4277556;
        let expected_ex2: i64 = 3263827;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 5552221122013;
        let expected_ex2: i64 = 11371597126232;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
