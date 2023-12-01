use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn ex1(file: &str) -> Result<u32> {
    let mut sum = 0;
    for l in file.lines() {

        let u1 = l
            .chars()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .ok_or_else(|| Error::msg("bad digit"))?;
        let u2 = l
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .ok_or_else(|| Error::msg("bad digit"))?;

        sum += u1 * 10 + u2;
    }
    Ok(sum)
}

fn ex2(file: &str) -> Result<u32> {
    let mut sum = 0;
    for l in file.lines() {

        let u1 = l
            .chars()
            .enumerate()
            .find_map(|(i, c)| num_at_pos(&l, i, c))
            .unwrap();
        let u2 = l
            .chars()
            .rev()
            .enumerate()
            .find_map(|(i, c)| num_at_pos(&l, l.len() - 1 - i, c))
            .unwrap();
        sum += u1 * 10 + u2;
    }
    Ok(sum)
}

fn num_at_pos(s: &str, pos: usize, c: char) -> Option<u32> {
    if c.is_numeric() {
        return c.to_digit(10);
    }
    if pos >= 2 {
        match &s[pos - 2..=pos] {
            "one" => {
                return Some(1);
            }
            "two" => {
                return Some(2);
            }
            "six" => {
                return Some(6);
            }
            _ => {}
        }
    }
    if pos >= 3 {
        match &s[pos - 3..=pos] {
            "four" => {
                return Some(4);
            }
            "five" => {
                return Some(5);
            }
            "nine" => {
                return Some(9);
            }
            _ => {}
        }
    }
    if pos >= 4 {
        match &s[pos - 4..=pos] {
            "three" => {
                return Some(3);
            }
            "seven" => {
                return Some(7);
            }
            "eight" => {
                return Some(8);
            }
            "nine" => {
                return Some(9);
            }
            _ => {
                return None;
            }
        }
    }
    None
}

fn main() {
    let file = fs::read_to_string("./inputs/d1_1.txt").unwrap();

    let start = Instant::now();
    let res_ex1 = ex1(&file);
    let duration = start.elapsed();
    println!("ex1: {} (computed in {:?})", res_ex1.unwrap(), duration);

    let start = Instant::now();
    let res_ex2 = ex2(&file);
    let duration = start.elapsed();
    println!("ex2: {} (computed in {:?})", res_ex2.unwrap(), duration);
}