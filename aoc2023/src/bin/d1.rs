use std::io::{prelude::*, BufReader};
use std::fs::File;

use anyhow::{Result, Error};

fn ex1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let l = line?;
        let mut num_str = String::new();
        for c in l.chars() {
            if c.is_numeric() {
                num_str.push(c);
                break;
            }
        }
        for c in l.chars().rev() {
            if c.is_numeric() {
                num_str.push(c);
                break;
            }
        }
        sum += num_str.parse::<u32>()?
    }
    Ok(sum)
}

fn ex2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let l = line?;
        let mut num_str = String::new();
        
        for (i, c) in l.chars().enumerate() {
            if let Some(num) = num_at_pos(&l, i, c) {
                num_str += num.to_string().as_str();
                break
            }
        }
        for (i, c) in l.chars().rev().enumerate() {
            if let Some(num) = num_at_pos(&l, l.len()-1-i, c) {
                num_str += num.to_string().as_str();
                break
            }
        }
        sum += num_str.parse::<u32>()?
    }
    Ok(sum)
}

fn num_at_pos(s: &str, pos: usize, c: char) -> Option<u32> {
    if c.is_numeric() {
        return c.to_digit(10);
    }
    if pos >= 2 {
        match &s[pos-2..=pos] {
            "one" => { return Some(1); },
            "two" => { return Some(2); },
            "six" => { return Some(6); },
            _ => {},
        }
    }
    if pos >= 3 {
        match &s[pos-3..=pos] {
            "four" => { return Some(4); },
            "five" => { return Some(5); },
            "nine" => { return Some(9); },
            _ => {},
        }
    }
    if pos >= 4 {
        match &s[pos-4..=pos] {
            "three" => { return Some(3); },
            "seven" => { return Some(7); },
            "eight" => { return Some(8); },
            "nine" => { return Some(9); },
            _ => { return None; },
        }
    }
    None
}

fn main() {
    println!("ex1: {}", ex1("inputs/d1_1.txt").unwrap());
    println!("ex2: {}", ex2("inputs/d1_1.txt").unwrap());
}
