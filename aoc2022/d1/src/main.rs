use std::fs::File;
use std::io::{BufReader, prelude::*};

fn elf_capacity_list(filename: &str) -> Result<Vec<i32>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut result = Vec::new();
    for line in reader.lines() {
        match line?.as_str() {
            "" => {
                result.push(count);
                count = 0;
            }
            l => count += l.parse::<i32>().unwrap(),
        }
    }
    Ok(result)
}

fn main() {
    let mut v = elf_capacity_list("input1.txt").unwrap();
    v.sort();
    println!("ex1: {}", v.last().unwrap());
    let last3sum: i32 = (&v[v.len()-3..]).iter().sum();
    println!("ex2: {}", last3sum);
}
