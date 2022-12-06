use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn is_start_of_packet(arr: &[u8]) -> bool {
    let mut s = HashSet::new();
    for c in arr {
        if s.contains(c) {
            return false;
        }
        s.insert(c);
    }
    true
}

fn ex(filename: &str, size: usize) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rolling_window = vec![0u8; size];
    // we just assume this is all ascii
    for (i, c) in reader.bytes().enumerate() {
        rolling_window[i % size] = c?;
        if i >= size-1 && is_start_of_packet(&rolling_window) {
            return Ok(i + 1);
        }
    }
    Ok(0)
}

fn main() {
    println!("ex1: {}", ex("inputs/d6_1.txt", 4).unwrap());
    println!("ex2: {}", ex("inputs/d6_1.txt", 14).unwrap());
}
