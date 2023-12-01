use std::io;
use std::io::BufReader;
use std::fs::File;

fn ex1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(0)
}

fn ex2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(0)
}

fn main() {
    println!("ex1: {}", ex1("inputs/dX_1.txt").unwrap());
    println!("ex2: {}", ex2("inputs/d3X_1.txt").unwrap());
}
