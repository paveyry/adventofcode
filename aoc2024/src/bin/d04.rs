use aoc2024::run_day;

use anyhow::{Context, Error, Result};
use itertools::{process_results, Itertools};

const DAY: &str = "d04";

static MAS: &[u8; 3] = &[b'M', b'A', b'S'];

struct Matrix(Vec<Vec<u8>>);

type Pos = (isize, isize);

impl Matrix {
    fn new(v: Vec<Vec<u8>>) -> Result<Self> {
        if v.is_empty() || v[0].is_empty() {
            Err(Error::msg("matrix has a dimension to 0"))
        } else {
            Ok(Self(v))
        }
    }

    fn from_file(file: &str) -> Result<Self> {
        Matrix::new(file.lines().map(|s| s.bytes().collect()).collect())
    }

    fn at(&self, p: Pos) -> Option<u8> {
        self.0.get(p.0 as usize)?.get(p.1 as usize).copied()
    }

    fn hlen(&self) -> isize {
        self.0.len() as isize
    }

    fn vlen(&self) -> isize {
        self.0[0].len() as isize
    }

    fn check_xmas_word(&self, p1: Pos, p2: Pos, p3: Pos) -> i64 {
        if let Some(true) = (|| Some([self.at(p1)?, self.at(p2)?, self.at(p3)?].eq(MAS)))() {
            1
        } else {
            0
        }
    }

    fn count_xmas_from_start(&self, (x, y): Pos) -> i64 {
        self.check_xmas_word((x - 1, y), (x - 2, y), (x - 3, y))
            + self.check_xmas_word((x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3))
            + self.check_xmas_word((x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3))
            + self.check_xmas_word((x + 1, y), (x + 2, y), (x + 3, y))
            + self.check_xmas_word((x + 1, y-1), (x + 2, y-2), (x + 3, y-3))
            + self.check_xmas_word((x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3))
            + self.check_xmas_word((x, y - 1), (x, y - 2), (x, y - 3))
            + self.check_xmas_word((x, y + 1), (x, y + 2), (x, y + 3))
    }

    fn is_cross_mas_from_start(&self, (x, y): Pos) -> Option<bool> {
        let top_left = self.at((x - 1, y - 1))?;
        let top_right = self.at((x + 1, y - 1))?;
        let bottom_left = self.at((x - 1, y + 1))?;
        let bottom_right = self.at((x + 1, y + 1))?;
        let diag1 =
            top_left == b'M' && bottom_right == b'S' || top_left == b'S' && bottom_right == b'M';
        let diag2 =
            top_right == b'M' && bottom_left == b'S' || top_right == b'S' && bottom_left == b'M';
        Some(diag1 && diag2)
    }
}

fn ex1(file: &str) -> Result<i64> {
    let matrix = Matrix::from_file(file)?;
    Ok((0..matrix.hlen())
        .cartesian_product(0..matrix.vlen())
        .filter(|(x, y)| matches!(matrix.at((*x, *y)), Some(b'X')))
        .map(|pos| matrix.count_xmas_from_start(pos))
        .sum())
}

fn ex2(file: &str) -> Result<i64> {
    let matrix = Matrix::from_file(file)?;
    process_results(
        (1..matrix.hlen() - 1)
            .cartesian_product(1..matrix.vlen() - 1)
            .filter(|(x, y)| matches!(matrix.at((*x, *y)), Some(b'A')))
            .map(|pos| matrix.is_cross_mas_from_start(pos).context("out of bounds"))
            .filter(|r| matches!(r, Err(_) | Ok(true))),
        |r| r.count() as i64,
    )
    .map_err(Into::into)
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
        let example1 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let example2 = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        let expected_ex1: i64 = 18;
        let expected_ex2: i64 = 9;
        assert_eq!(expected_ex1, ex1(example1).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example2).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 2593;
        let expected_ex2: i64 = 1950;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
