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

#[derive(Clone)]
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
    start_pos: (usize, usize),
}

impl Map {
    fn new(m: Vec<Vec<Tile>>, start_pos: (usize, usize)) -> Result<Self> {
        if m.is_empty() || m[0].is_empty() {
            return Err(Error::msg("missing map data"));
        }
        let height = m.len();
        let width = m[0].len();
        Ok(Self {
            m,
            height,
            width,
            start_pos,
        })
    }

    fn from_file(file: &str) -> Result<Self> {
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
            start_pos,
        )?;
        Ok(map)
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

    fn iter(&self) -> PathIterator {
        PathIterator {
            map: self,
            cur: self.start_pos,
            prev: Cardinal::default(),
            first_passed: false,
        }
    }
}

struct PathIterator<'a> {
    map: &'a Map,
    cur: (usize, usize),
    prev: Cardinal,
    first_passed: bool,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.cur;
        if let Some((x, y, prev)) = match self.map.m[i][j] {
            Tile::Ground => None,
            Tile::Start => {
                if !self.first_passed {
                    self.first_passed = true;
                    self.map.first_node_from_start(i, j)
                } else {
                    None
                }
            }
            Tile::Pipe((c1, c2)) => {
                if c1 == self.prev {
                    self.map.next_from_cardinal(i, j, c2)
                } else if c2 == self.prev {
                    self.map.next_from_cardinal(i, j, c1)
                } else {
                    None
                }
            }
        } {
            self.cur = (x, y);
            self.prev = prev;
            Some((x, y))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn print_path_map(m: &Vec<Vec<bool>>) {
    let mut s = String::with_capacity(m.len() * (m[0].len() + 1));
    for l in m {
        for c in l {
            if *c {
                s.push('+');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    print!("{s}");
}

fn ex1(file: &str) -> Result<usize> {
    let map = Map::from_file(file)?;
    Ok(map.iter().count() / 2)
}

fn ex2(file: &str) -> Result<usize> {
    let map = Map::from_file(file)?;
    let mut pth_map = vec![vec![false; map.width]; map.height];
    for (i, j) in map.iter() {
        pth_map[i][j] = true;
    }
    let mut count = 0;
    let mut is_start_northbound = false;
    if map.start_pos.0 > 0 {
        if let Tile::Pipe((c1, c2)) = map.m[map.start_pos.0 - 1][map.start_pos.1] {
            if [c1, c2].contains(&Cardinal::South) {
                is_start_northbound = true;
            }
        }
    }
    for (i, l) in pth_map.iter().enumerate() {
        let mut inside = false;
        for (j, b) in l.iter().enumerate() {
            if !*b {
                if inside {
                    count += 1;
                }
            } else {
                let n = map.m[i][j].clone();
                if let Tile::Pipe((c1, c2)) = n {
                    if [c1, c2].contains(&Cardinal::North) {
                        inside = !inside
                    }
                } else if matches!(Tile::Start, _n) && is_start_northbound {
                    inside = !inside;
                }
            }
        }
    }
    Ok(count)
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
    }

    #[test]
    fn test_ex2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, ex2(input).unwrap());
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, ex2(input).unwrap());
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d10_1.txt").unwrap();
        assert_eq!(6951, ex1(&file).unwrap());
        assert_eq!(563, ex2(&file).unwrap());
    }
}
