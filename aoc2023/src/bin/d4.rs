use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn count_line_winning_nums<'a>(l: &'a str, winning_nums_set: &mut HashSet<&'a str>) -> Result<u32> {
    let mut iter = l
        .split(": ")
        .nth(1)
        .ok_or_else(|| Error::msg("bad format"))?
        .split(" | ");
    iter.next()
        .ok_or_else(|| Error::msg("bad format"))?
        .split_whitespace()
        .for_each(|n| {
            winning_nums_set.insert(n);
        });
    let count = iter
        .next()
        .ok_or_else(|| Error::msg("bad format"))?
        .split_whitespace()
        .filter(|n| winning_nums_set.contains(n))
        .count() as u32;
    winning_nums_set.clear();
    Ok(count)
}

fn ex1(file: &str) -> Result<u32> {
    let mut winning_nums_set = HashSet::new();
    Ok(file
        .lines()
        .map(|l| count_line_winning_nums(l, &mut winning_nums_set))
        .filter_map(|n| n.ok())
        .filter(|n| *n > 0)
        .map(|n| u32::pow(2, n - 1))
        .sum())
}

fn ex2(file: &str) -> Result<u32> {
    let mut winning_nums_set = HashSet::new();
    let scores_per_line = file
        .lines()
        .map(|l| count_line_winning_nums(l, &mut winning_nums_set))
        .collect::<Result<Vec<_>>>()?;
    let mut owned_cards = vec![1; scores_per_line.len()];
    let mut total = 0;
    for i in 0..owned_cards.len() {
        let mut owned = owned_cards[i];
        total += owned;
        let c = scores_per_line[i];
        while owned > 0 {
            for j in i + 1..=i + c as usize {
                owned_cards[j] += 1;
            }
            owned -= 1;
        }
    }
    Ok(total)
}

fn main() {
    let file = fs::read_to_string("./inputs/d4_1.txt").unwrap();

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
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(13, ex1(input).unwrap());
        assert_eq!(30, ex2(input).unwrap());
    }
}
