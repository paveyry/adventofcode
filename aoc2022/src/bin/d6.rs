use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn is_start_of_packet(arr: &[u8]) -> bool {
    let mut s = HashSet::new();
    for c in arr {
        match s.get(c) {
            Some(_) => {
                return false;
            }
            None => {
                s.insert(c);
            }
        }
    }
    return true;
}

fn ex2(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rolling_window = [0u8; 14];
    // we just assume this is all ascii
    for (i, c) in reader.bytes().enumerate() {
        rolling_window[i % 14] = c?;
        if i >= 13 && is_start_of_packet(&rolling_window) {
            return Ok(i + 1);
        }
    }
    Ok(0)
}

fn ex1(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rolling_window = [0u8; 4];
    // we just assume this is all ascii
    for (i, c) in reader.bytes().enumerate() {
        rolling_window[i % 4] = c?;
        if i >= 3 && is_start_of_packet(&rolling_window) {
            return Ok(i + 1);
        }
    }
    Ok(0)
}

fn main() {
    println!("ex1: {}", ex1("inputs/d6_1.txt").unwrap());
    println!("ex2: {}", ex2("inputs/d6_1.txt").unwrap());
}
