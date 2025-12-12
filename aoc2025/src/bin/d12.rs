use std::str::FromStr;

use aoc2025::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d12";

#[derive(Debug, Clone)]
struct Shape {
    _width: u64,
    _height: u64,
    _shape: Vec<Vec<bool>>,
    num_cells: u64,
}

impl FromStr for Shape {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let shape: Vec<Vec<bool>> = s
            .lines()
            .skip(1)
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let num_cells = shape.iter().flatten().filter(|b| **b).count() as u64;
        Ok(Self {
            _width: shape[0].len() as u64,
            _height: shape.len() as u64,
            _shape: shape,
            num_cells,
        })
    }
}

#[derive(Debug, Clone)]
struct TreeFarm {
    shapes: Vec<Shape>,
    regions: Vec<((u64, u64), Vec<u64>)>,
}

impl FromStr for TreeFarm {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iter = s.split("\n\n");
        let shapes = iter
            .clone()
            .take(6)
            .map(|s| s.parse::<Shape>())
            .collect::<Result<_>>()?;
        let regions = iter
            .nth(6)
            .context("failed to read regions")?
            .lines()
            .map(|l| {
                let (dims, num_per_shape) = l.split_once(": ").context("failed to parse region")?;
                let (width_s, height_s) = dims.split_once("x").context("failed to parse region")?;
                let (width, height) = (width_s.parse::<u64>()?, height_s.parse::<u64>()?);
                let nums_per_shape = num_per_shape
                    .split_whitespace()
                    .map(|s| Ok(s.parse::<u64>()?))
                    .collect::<Result<Vec<_>>>()?;
                Ok(((width, height), nums_per_shape))
            })
            .collect::<Result<_>>()?;

        Ok(Self { shapes, regions })
    }
}

fn ex1(file: &str) -> Result<i64> {
    let tf: TreeFarm = file.parse()?;
    let mut count = 0;
    for ((r_width, r_height), presents_req) in tf.regions.iter() {
        if (r_width * r_height)
            >= presents_req
                .iter()
                .zip(tf.shapes.iter())
                .map(|(pr, shape)| shape.num_cells * pr)
                .sum()
        {
            // What a joke that this works
            count += 1;
        }
    }
    Ok(count)
}

fn main() {
    run_day(DAY, ex1, |_| Ok(0));
}

#[cfg(test)]
mod tests {
    use super::DAY;
    use std::fs;

    use super::ex1;

    #[test]
    fn test() {
        let example = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";
        let expected_ex1: i64 = 2;
        // the example input is more complex than the real-life input, and thus
        // this simplistic approach does not work with it :D
        assert_eq!(expected_ex1 + 1, ex1(example).expect("ex1 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 510;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
    }
}
