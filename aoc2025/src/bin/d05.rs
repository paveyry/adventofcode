use aoc2025::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d05";

#[allow(clippy::type_complexity)]
fn parse_input(file: &str, get_ing_list: bool) -> Result<(Vec<(i64, i64)>, Vec<i64>)> {
    let mut parts = file.split("\n\n");
    let ranges = parts
        .next()
        .context("failed to find ranges section")?
        .lines()
        .filter_map(|l| l.split_once('-'))
        .map(|(a, b)| Ok((a.parse::<i64>()?, b.parse::<i64>()?)))
        .collect::<Result<Vec<(i64, i64)>>>()?;
    if !get_ing_list {
        return Ok((ranges, Vec::new()));
    }
    let ingredients = parts
        .next()
        .context("failed to find ingredients section")?
        .lines()
        .map(|l| Ok(l.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()?;
    Ok((ranges, ingredients))
}

fn ex1(file: &str) -> Result<i64> {
    let (ranges, ingredients) = parse_input(file, true)?;
    let r = ingredients
        .iter()
        .filter(|ing| ranges.iter().any(|(a, b)| (a..=b).contains(ing)))
        .count();

    Ok(r as i64)
}

fn ex2(file: &str) -> Result<i64> {
    let (mut ranges, _) = parse_input(file, false)?;
    let mut count = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut iter = ranges.iter().enumerate();
    let mut prev = *iter.next().context("failed to extract first element")?.1;
    for (i, (start, end)) in iter {
        let mut merged = false;
        if *start <= prev.1 + 1 {
            merged = true;
            if *end > prev.1 {
                prev = (prev.0, *end);
            }
        }

        if !merged || i == ranges.len() - 1 {
            count += prev.1 - prev.0 + 1;
            prev = (*start, *end);
        }
    }
    Ok(count)
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
        let example = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let expected_ex1: i64 = 3;
        let expected_ex2: i64 = 14;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_ex2_extra_edgecases() {
        let example = "3-5
10-14
16-20
12-18
17-21
3-4
13-14

1
";
        let expected_ex2: i64 = 15;
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 567;
        let expected_ex2: i64 = 354149806372909;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
