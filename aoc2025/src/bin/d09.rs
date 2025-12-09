use std::cmp::{max, min};
use std::collections::HashMap;

use aoc2025::run_day;

use anyhow::{Context, Result};
use itertools::Itertools;

const DAY: &str = "d09";

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area(&self, other: &Self) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({},{})", self.x, self.y).as_str())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TilePair {
    t1: Tile,
    t2: Tile,
}

impl TilePair {
    fn new(t1: Tile, t2: Tile) -> Self {
        if t1 < t2 {
            Self { t1, t2 }
        } else {
            Self { t1: t2, t2: t1 }
        }
    }

    fn rect_coordinates(&self) -> (i64, i64, i64, i64) {
        let (t1, t2) = (&self.t1, &self.t2);
        (
            min(t1.x, t2.x),
            max(t1.x, t2.x),
            min(t1.y, t2.y),
            max(t1.y, t2.y),
        )
    }
}

impl std::fmt::Display for TilePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[{} - {}]", self.t1, self.t2).as_str())
    }
}

fn parse_input(file: &str) -> Result<Vec<Tile>> {
    file.lines()
        .filter_map(|l| l.split_once(","))
        .map(|(x, y)| Ok(Tile::new(x.parse::<i64>()?, y.parse::<i64>()?)))
        .collect()
}

fn compute_areas_ex1(tiles: &[Tile]) -> HashMap<TilePair, i64> {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .iter()
                .enumerate()
                .filter(move |(j, _b)| i != *j)
                .map(|(_j, b)| (TilePair::new(a.clone(), b.clone()), a.area(b)))
        })
        .collect()
}

fn ex1(file: &str) -> Result<i64> {
    let tiles = parse_input(file)?;
    let areas = compute_areas_ex1(tiles.as_slice());
    areas
        .values()
        .max()
        .map(ToOwned::to_owned)
        .context("failed to find max")
}

fn ex2(file: &str) -> Result<i64> {
    let mut tiles = parse_input(file)?;
    tiles.push(tiles[0].clone());

    let areas = compute_areas_ex1(tiles.as_slice());
    let areas_vec = areas
        .iter()
        .map(|(tp, a)| (tp.clone(), *a))
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect::<Vec<_>>();

    for (rect, area) in areas_vec {
        let mut intersects_with_segment = false;
        for (t1, t2) in tiles.iter().tuple_windows() {
            let (rxmin, rxmax, rymin, rymax) = rect.rect_coordinates();
            if t1.x == t2.x {
                let ymin = min(t1.y, t2.y);
                let ymax = max(t1.y, t2.y);
                if ymin < rymax && ymax > rymin && t1.x > rxmin && t1.x < rxmax {
                    intersects_with_segment = true;
                    break;
                }
            } else {
                let xmin = min(t1.x, t2.x);
                let xmax = max(t1.x, t2.x);
                if xmin < rxmax && xmax > rxmin && t1.y > rymin && t1.y < rymax {
                    intersects_with_segment = true;
                    break;
                }
            }
        }
        if !intersects_with_segment {
            return Ok(area);
        }
    }
    panic!("failed to find working rectangle");
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
        let example = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
        let expected_ex1: i64 = 50;
        let expected_ex2: i64 = 24;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 4782151432;
        let expected_ex2: i64 = 1450414119;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
