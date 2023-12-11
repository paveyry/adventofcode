use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn distance(g1: (i64, i64), g2: (i64, i64)) -> i64 {
    (g2.1 - g1.1).abs() + (g2.0 - g1.0).abs()
}

fn galaxy_distances(file: &str, multiplying_scale: i64) -> Result<i64> {
    let mut column_counts = vec![
        0;
        file.lines()
            .next()
            .ok_or_else(|| Error::msg("missing first line"))?
            .len()
    ];

    let mut galaxies = Vec::new();

    let mut i = 0i64;
    for l in file.lines() {
        let mut has_galaxy = false;
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                has_galaxy = true;
                column_counts[j] += 1;
                galaxies.push((i, j as i64));
            }
        }
        if !has_galaxy {
            i += multiplying_scale - 1;
        }
        i += 1;
    }
    let mut increased_count = 0;
    for (j, c) in column_counts.into_iter().enumerate() {
        if c == 0 {
            for g in galaxies.iter_mut() {
                if g.1 - increased_count > j as i64 {
                    g.1 += multiplying_scale - 1;
                }
            }
            increased_count += multiplying_scale - 1;
        }
    }
    let mut sum = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            let d = distance(*g1, *g2);
            sum += d;
        }
    }
    Ok(sum)
}

fn ex1(file: &str) -> Result<i64> {
    galaxy_distances(file, 2)
}

fn ex2(file: &str, scale: i64) -> Result<i64> {
    galaxy_distances(file, scale)
}

fn main() {
    let file = fs::read_to_string("./inputs/d11_1.txt").unwrap();

    let start = Instant::now();
    let res_ex1 = ex1(&file);
    let duration = start.elapsed();
    println!("ex1: {} (computed in {:?})", res_ex1.unwrap(), duration);

    let start = Instant::now();
    let res_ex2 = ex2(&file, 1000000);
    let duration = start.elapsed();
    println!("ex2: {} (computed in {:?})", res_ex2.unwrap(), duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, ex1(input).unwrap());
        assert_eq!(1030, ex2(input, 10).unwrap());
        assert_eq!(8410, ex2(input, 100).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d11_1.txt").unwrap();
        assert_eq!(9445168, ex1(&file).unwrap());
        assert_eq!(742305960572, ex2(&file, 1000000).unwrap());
    }
}
