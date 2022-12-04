use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

fn ex2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let range_str_pairs: Vec<String> = line?.split(',').map(str::to_string).collect();
        let range1: Vec<u32> = range_str_pairs[0]
            .split('-')
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .collect();
        let range2: Vec<u32> = range_str_pairs[1]
            .split('-')
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .collect();
        if range1[0] <= range2[1] && range1[1] >= range2[0]
            || range2[0] <= range1[1] && range2[1] >= range1[0]
        {
            result += 1;
        }
    }
    Ok(result)
}

fn ex1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let range_str_pairs: Vec<String> = line?.split(',').map(str::to_string).collect();
        let range1: Vec<u32> = range_str_pairs[0]
            .split('-')
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .collect();
        let range2: Vec<u32> = range_str_pairs[1]
            .split('-')
            .map(str::parse::<u32>)
            .filter_map(Result::ok)
            .collect();
        if range1[0] <= range2[0] && range1[1] >= range2[1]
            || range2[0] <= range1[0] && range2[1] >= range1[1]
        {
            result += 1;
        }
    }
    Ok(result)
}

fn main() {
    println!("ex1: {}", ex1("inputs/d4_1.txt").unwrap());
    println!("ex2: {}", ex2("inputs/d4_1.txt").unwrap());
}
