use std::collections::HashSet;

use aoc2025::run_day;

use anyhow::Result;

const DAY: &str = "d04";

fn parse_grid(file: &str) -> Vec<Vec<bool>> {
    file.lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect()
}

fn grid_to_coord_set(grid: &[Vec<bool>]) -> HashSet<(isize, isize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, val)| **val)
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect()
}

fn ex1(file: &str) -> Result<i64> {
    let set = grid_to_coord_set(parse_grid(file).as_ref());
    let count = set
        .iter()
        .filter(|(x, y)| {
            (x - 1..=x + 1)
                .flat_map(|i| (y - 1..=y + 1).map(move |j| (i, j)))
                .filter(|(i, j)| (i, j) != (x, y) && set.contains(&(*i, *j)))
                .count()
                < 4
        })
        .count();
    Ok(count as i64)
}

fn ex2(file: &str) -> Result<i64> {
    let mut set = grid_to_coord_set(parse_grid(file).as_ref());
    let mut total = 0;
    loop {
        let mut count = 0;
        for (x, y) in set.clone().iter() {
            let neighbours = (x - 1..=x + 1)
                .flat_map(|i| (y - 1..=y + 1).map(move |j| (i, j)))
                .filter(|(i, j)| (i, j) != (x, y) && set.contains(&(*i, *j)))
                .count();
            if neighbours < 4 {
                count += 1;
                set.remove(&(*x, *y));
            }
        }
        if count == 0 {
            break;
        }
        total += count;
    }

    Ok(total)
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
        let example = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        let expected_ex1: i64 = 13;
        let expected_ex2: i64 = 43;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 1384;
        let expected_ex2: i64 = 8013;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
