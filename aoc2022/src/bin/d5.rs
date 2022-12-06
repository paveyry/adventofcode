use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn parse_schema(lines: &[String], indexes: &BTreeSet<usize>) -> Vec<VecDeque<char>> {
    let mut res = vec![VecDeque::new(); indexes.len()];
    for l in lines.iter().rev() {
        let chars: Vec<char> = l
            .chars()
            .enumerate()
            .filter(|(ind, _)| indexes.contains(ind))
            .map(|(_, c)| c)
            .collect();
        for (i, c) in chars.iter().enumerate() {
            if !c.is_whitespace() {
                res[i].push_back(*c);
            }
        }
    }

    res
}

fn compute_indexes(l: &str) -> BTreeSet<usize> {
    let mut hs = BTreeSet::new();
    for (i, c) in l.chars().enumerate() {
        if c.is_whitespace() {
            continue;
        }
        hs.insert(i);
    }
    hs
}

fn apply_move(schema: &mut [VecDeque<char>], qty: usize, src: usize, dst: usize, with_mud: bool) {
    let mut moved_crates = Vec::new();
    for _ in 0..qty {
        moved_crates.push(schema[src].pop_back().unwrap());
    }
    if with_mud {
        for c in moved_crates.iter().rev() {
            schema[dst].push_back(*c);
        }
        return;
    }
    for c in &moved_crates {
        schema[dst].push_back(*c);
    }
}

fn ex(filename: &str, with_mud: bool) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut schema_strings = Vec::new();
    let mut schema: Option<Vec<VecDeque<char>>> = None;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if line.starts_with(" 1") {
            let indexes = compute_indexes(line.as_str());
            schema = Some(parse_schema(&schema_strings, &indexes));
            continue;
        }
        if schema.is_none() {
            schema_strings.push(line);
            continue;
        }
        let schema = schema.as_mut().unwrap(); // we've continued if None
        let words: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let qty = words[1].parse::<usize>().unwrap();
        let src = words[3].parse::<usize>().unwrap() - 1;
        let dst = words[5].parse::<usize>().unwrap() - 1;
        apply_move(schema, qty, src, dst, with_mud);
    }
    for v in schema.unwrap() {
        print!("{}", v.back().unwrap());
    }
    println!();

    Ok(0)
}

fn main() {
    println!("ex1: {}", ex("inputs/d5_1.txt", false).unwrap());
    println!("ex2: {}", ex("inputs/d5_1.txt", true).unwrap());
}
