use std::collections::{HashMap, HashSet};

use aoc2025::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d07";

fn parse_input(file: &str) -> Result<(usize, Vec<Vec<bool>>)> {
    let mut iter = file.lines();
    let ray = iter
        .next()
        .context("failed to read first line")?
        .find("S")
        .context("failed to find beam start")?;
    // first line is skipped but it is not needed
    let grid: Vec<Vec<_>> = iter
        .map(|l| l.chars().map(|c| c == '^').collect())
        .collect();
    Ok((ray, grid))
}

fn ex1(file: &str) -> Result<i64> {
    let (start_ray, grid) = parse_input(file)?;
    let mut rays = HashSet::new();
    rays.insert(start_ray);

    let mut split_count = 0;
    for row in grid.iter() {
        let cur_rays = rays.clone();
        for ray in cur_rays {
            if row[ray] {
                rays.remove(&ray);
                if ray > 0 {
                    rays.insert(ray - 1);
                }
                if ray < row.len() - 1 {
                    rays.insert(ray + 1);
                }
                split_count += 1;
            }
        }
    }
    Ok(split_count)
}

fn ex2(file: &str) -> Result<i64> {
    let (start_ray, grid) = parse_input(file)?;
    let mut rays = HashMap::new();
    rays.insert(start_ray, 1); // Value is the number of timelines leading to that ray

    for row in grid.iter() {
        let cur_rays = rays.clone();
        for (ray, timelines) in cur_rays {
            if row[ray] {
                rays.remove(&ray);
                if ray > 0 {
                    *rays.entry(ray - 1).or_default() += timelines;
                }
                if ray < row.len() - 1 {
                    *rays.entry(ray + 1).or_default() += timelines;
                }
            }
        }
    }
    Ok(rays.values().sum::<i64>())
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
        let example = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let expected_ex1: i64 = 21;
        let expected_ex2: i64 = 40;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 1560;
        let expected_ex2: i64 = 25592971184998;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
