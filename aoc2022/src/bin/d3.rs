use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn byte_to_priority(b: u8) -> u32 {
    match b {
        b'a'..=b'z' => u32::from(b - b'a') + 1,
        b'A'..=b'Z' => u32::from(b - b'A') + 27,
        _ => panic!(),
    }
}

fn ex2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut hm: HashMap<u8, [bool; 3]> = HashMap::new();
    let mut common_char = 0;
    for (i, line) in reader.lines().enumerate() {
        let l = line?;
        let lb = l.as_bytes();
        for c in lb {
            if let Some(v) = hm.get_mut(c) {
                v[i % 3] = true;
                if i % 3 == 2 && v.iter().all(|x| *x) {
                    common_char = *c;
                    break;
                }
                continue;
            }
            let mut v = [false, false, false];
            v[i % 3] = true;
            hm.insert(*c, v);
        }
        if i % 3 == 2 {
            result += byte_to_priority(common_char);
            hm.clear();
        }
    }
    Ok(result)
}

fn ex1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let l = line?;
        let lb = l.as_bytes();
        let mut hs = HashSet::new();
        for c in &lb[..lb.len() / 2] {
            hs.insert(c);
        }
        let mut duplicate = 0;
        for c in &lb[lb.len() / 2..] {
            if hs.contains(c) {
                duplicate = *c;
                break;
            }
        }
        result += byte_to_priority(duplicate);
    }
    Ok(result)
}

fn main() {
    println!("ex1: {}", ex1("inputs/d3_1.txt").unwrap());
    println!("ex2: {}", ex2("inputs/d3_1.txt").unwrap());
}
