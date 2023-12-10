use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
enum Cardinal {
    #[default]
    North,
    West,
    South,
    East,
}

enum Tile {
    Ground,
    Start,
    Pipe((Cardinal, Cardinal)),
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '|' => Ok(Tile::Pipe((Cardinal::North, Cardinal::South))),
            '-' => Ok(Tile::Pipe((Cardinal::East, Cardinal::West))),
            'L' => Ok(Tile::Pipe((Cardinal::North, Cardinal::East))),
            'J' => Ok(Tile::Pipe((Cardinal::North, Cardinal::West))),
            '7' => Ok(Tile::Pipe((Cardinal::South, Cardinal::West))),
            'F' => Ok(Tile::Pipe((Cardinal::South, Cardinal::East))),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err(Error::msg("invalid tile char")),
        }
    }
}

impl Tile {
    fn connects(&self, relative_pos_of_other_tile: Cardinal) -> bool {
        if let Tile::Pipe((c1, c2)) = self {
            if *c1 == relative_pos_of_other_tile || *c2 == relative_pos_of_other_tile {
                return true;
            }
        }
        false
    }
}

struct Map {
    m: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(m: Vec<Vec<Tile>>) -> Result<Self> {
        if m.is_empty() || m[0].is_empty() {
            return Err(Error::msg("missing map data"));
        }
        let height = m.len();
        let width = m[0].len();
        Ok(Self {
            m,
            height,
            width,
        })
    }

    /// Returns the first matching node found, doesn't check if there is a second
    fn first_node_from_start(&self, i: usize, j: usize) -> Option<(usize, usize, Cardinal)> {
        if i > 0 && self.m[i - 1][j].connects(Cardinal::South) {
            return Some((i - 1, j, Cardinal::South));
        }
        if i < self.height - 1 && self.m[i + 1][j].connects(Cardinal::North) {
            return Some((i + 1, j, Cardinal::North));
        }
        if j > 0 && self.m[i][j - 1].connects(Cardinal::East) {
            return Some((i + 1, j, Cardinal::East));
        }
        if j < self.width - 1 && self.m[i][j + 1].connects(Cardinal::West) {
            return Some((i + 1, j, Cardinal::West));
        }
        None
    }

    fn validate(&self, i: usize, j: usize, origin: Cardinal) -> Option<(usize, usize, Cardinal)> {
        if i < self.height && j < self.width {
            Some((i, j, origin))
        } else {
            None
        }
    }

    // Returns the next node from pos (i,j) following the Cardinal direction, along with the opposite cardinal
    // direction (direction to origin)
    fn next_from_cardinal(
        &self,
        i: usize,
        j: usize,
        direction: Cardinal,
    ) -> Option<(usize, usize, Cardinal)> {
        match direction {
            Cardinal::East => self.validate(i, j + 1, Cardinal::West),
            Cardinal::West => self.validate(i, j - 1, Cardinal::East),
            Cardinal::North => self.validate(i - 1, j, Cardinal::South),
            Cardinal::South => self.validate(i + 1, j, Cardinal::North),
        }
    }

    fn next_node(&self, i: usize, j: usize, prev: Cardinal) -> Option<(usize, usize, Cardinal)> {
        match self.m[i][j] {
            Tile::Ground => None,
            Tile::Start => self.first_node_from_start(i, j),
            Tile::Pipe((c1, c2)) => {
                if c1 == prev {
                    self.next_from_cardinal(i, j, c2)
                } else if c2 == prev {
                    self.next_from_cardinal(i, j, c1)
                } else {
                    None
                }
            }
        }
    }
}

fn ex1(file: &str) -> Result<i64> {
    let mut start_pos = (0, 0);
    let map = Map::new(
        file.lines()
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let t = c.try_into();
                        if let Ok(Tile::Start) = t {
                            start_pos = (i, j);
                        }
                        t
                    })
                    .collect::<Result<Vec<Tile>>>()
            })
            .collect::<Result<Vec<_>>>()?,
    )?;
    let mut count = 0;
    let (mut i, mut j) = start_pos;
    let mut prev_cardinal = Cardinal::default();
    loop {
        let n = map.next_node(i, j, prev_cardinal);
        if let Some((x, y, c)) = n {
            // println!("({i},{j}, {prev_cardinal:?}) -> ({x}, {y}, {c:?})");
            if (x, y) == start_pos {
                return Ok((count + 1) / 2);
            }
            i = x;
            j = y;
            count += 1;
            prev_cardinal = c;
        } else {
            return Err(Error::msg(format!("node ({i},{j}) didn't find next")));
        }
    }
}

fn ex2(file: &str) -> Result<i64> {
    Ok(0)
}

fn main() {
    let file = fs::read_to_string("./inputs/d10_1.txt").unwrap();

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
    fn test_ex1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, ex1(input).unwrap());
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, ex1(input).unwrap());
        // assert_eq!(2, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d10_1.txt").unwrap();
        assert_eq!(6951, ex1(&file).unwrap());
        // assert_eq!(1211, ex2(&file).unwrap());
    }
}
