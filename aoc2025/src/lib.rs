use std::fs;
use std::time::Instant;

use anyhow::Result;

pub fn run_day<F1, F2>(day: &str, ex1: F1, ex2: F2)
where
    F1: FnOnce(&str) -> Result<i64>,
    F2: FnOnce(&str) -> Result<i64>,
{
    let file =
        fs::read_to_string(format!("./inputs/{day}_1.txt")).expect("failed to read input file");

    let start = Instant::now();
    let res_ex1 = ex1(&file);
    let duration = start.elapsed();
    println!(
        "ex1: {} (computed in {:?})",
        res_ex1.expect("ex1 failed"),
        duration
    );

    let start = Instant::now();
    let res_ex2 = ex2(&file);
    let duration = start.elapsed();
    println!(
        "ex2: {} (computed in {:?})",
        res_ex2.expect("ex2 failed"),
        duration
    );
}
