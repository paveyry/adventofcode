use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use num::integer::lcm;
use anyhow::{Error, Result};

enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'R' => Ok(Instruction::Right),
            'L' => Ok(Instruction::Left),
            _ => Err(Error::msg("invalid instruction")),
        }
    }
}

fn ex1(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let instructions = lines
        .next()
        .ok_or_else(|| Error::msg("missing first line"))?
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<Instruction>>>()?;

    let mut cur: &str = Default::default();

    let m = lines
        .skip(1)
        .map(|l| {
            let v = (&l[0..=2], (&l[7..=9], &l[12..=14]));
            if v.0 == "AAA" {
                cur = v.0;
            }
            v
        })
        .collect::<HashMap<_, _>>();

    for (i, instruction) in instructions.iter().cycle().enumerate() {
        let pair = m
            .get(cur)
            .ok_or_else(|| Error::msg("missing entry from map"))?;
        let next = if let Instruction::Left = instruction {
            pair.0
        } else {
            pair.1
        };
        if next == "ZZZ" {
            return Ok(i as u64 + 1);
        }
        cur = next;
    }

    Err(Error::msg("no solution found"))
}

fn ex2(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let instructions = lines
        .next()
        .ok_or_else(|| Error::msg("missing first line"))?
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<Instruction>>>()?;

    let mut cur_keys = Vec::new();

    let m = lines
        .skip(1)
        .map(|l| {
            let v = (&l[0..=2], (&l[7..=9], &l[12..=14]));
            if v.0.ends_with('A') {
                cur_keys.push(v.0);
            }
            v
        })
        .collect::<HashMap<_, _>>();

    let mut counts = Vec::with_capacity(cur_keys.len());
    for cur in cur_keys.iter_mut() {
        for (i, instruction) in instructions.iter().cycle().enumerate() {
            let pair = m
                .get(cur)
                .ok_or_else(|| Error::msg("missing entry from map"))?;
            let next = if let Instruction::Left = instruction {
                pair.0
            } else {
                pair.1
            };
            if next.ends_with('Z') {
                counts.push(i as u64 + 1);
                break;
            }
            *cur = next;
        }
    }

    let mut it = counts.iter();
    let mut least_common_mult = *it.next().ok_or_else(|| Error::msg("missing counts"))?;
    for v in it {
        least_common_mult = lcm(least_common_mult, *v);
    }
    Ok(least_common_mult)
}

fn main() {
    let file = fs::read_to_string("./inputs/d8_1.txt").unwrap();

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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, ex1(input).unwrap());
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, ex1(input).unwrap());
    }
    #[test]
    fn test_ex2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d8_1.txt").unwrap();
        assert_eq!(12643, ex1(&file).unwrap());
        assert_eq!(13133452426987, ex2(&file).unwrap());
    }
}
