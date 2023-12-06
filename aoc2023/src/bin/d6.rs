use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn ex1(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let times = lines
        .next()
        .ok_or_else(|| Error::msg("missing first line"))?
        .split(':')
        .nth(1)
        .ok_or_else(|| Error::msg("bad first line"))?
        .split_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok);
    let distances = lines
        .next()
        .ok_or_else(|| Error::msg("missing second line"))?
        .split(':')
        .nth(1)
        .ok_or_else(|| Error::msg("bad second line"))?
        .split_whitespace()
        .map(str::parse::<u64>)
        .filter_map(Result::ok);
    Ok(times
        .zip(distances)
        .map(|(max_time, distance)| {
            (1..max_time)
                .filter(|t| (max_time - t) * t > distance)
                .count() as u64
        })
        .product::<u64>())
}

fn ex2(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let time = lines
        .next()
        .ok_or_else(|| Error::msg("missing first line"))?
        .split(':')
        .nth(1)
        .ok_or_else(|| Error::msg("bad first line"))?
        .replace(' ', "")
        .parse::<f64>()?;
    let distance = lines
        .next()
        .ok_or_else(|| Error::msg("missing second line"))?
        .split(':')
        .nth(1)
        .ok_or_else(|| Error::msg("bad second line"))?
        .replace(' ', "")
        .parse::<f64>()?;

    let delta_sqrt = (time * time - 4. * distance).sqrt();
    let mut x1 = ((time - delta_sqrt) / 2.).ceil();
    let mut x2 = ((time + delta_sqrt) / 2.).floor();
    if x1 * (time - x1) == distance {
        x1 += 1.;
    }
    if x2 * (time - x2) == distance {
        x2 -= 1.;
    }
    Ok((x2 - x1) as u64 + 1)
}

fn main() {
    let file = fs::read_to_string("./inputs/d6_1.txt").unwrap();

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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, ex1(input).unwrap());
        assert_eq!(71503, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d6_1.txt").unwrap();
        assert_eq!(1413720, ex1(&file).unwrap());
        assert_eq!(30565288, ex2(&file).unwrap());
    }
}
