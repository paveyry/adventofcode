use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn ex1(file: &str) -> Result<u32> {
    Ok(0)
}

fn ex2(file: &str) -> Result<u32> {
    Ok(0)
}

fn main() {
    let file = fs::read_to_string("./inputs/dX_X.txt").unwrap();

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
mod tests {}
