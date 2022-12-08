use std::collections::{HashMap, hash_map::Entry};
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum FSEntry {
    File(u32),
    Dir(String),
}

#[derive(Debug)]
struct Dir {
    content: Vec<FSEntry>,
}

fn build_filesystem(filename: &str) -> io::Result<HashMap<Vec<String>, Dir>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut current_dir: Vec<String> = Vec::new();
    let mut fs: HashMap<Vec<String>, Dir> = HashMap::new();
    fs.insert(vec!["/".to_string()], Dir {
        content: Vec::new(),
    });
    for (i, line) in reader.lines().enumerate() {
        let line = line?.to_string();
        let words: Vec<&str> = line.split_whitespace().collect();
        match &words[..] {
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", dir_name] => {
                if (*dir_name).eq("/") {
                    current_dir.clear();
                }
                current_dir.push((*dir_name).to_string());
            },
            ["$", "ls"] => {},
            ["dir", dir_name] => {
                let mut subdirpath = current_dir.clone();
                subdirpath.push((*dir_name).to_string());
                fs.insert(subdirpath, Dir { 
                    content: Vec::new() 
                });
                match fs.entry(current_dir.clone()) {
                    Entry::Occupied(ref mut o) =>  {
                        o.get_mut().content.push(FSEntry::Dir((*dir_name).to_string()));
                    }
                    Entry::Vacant(_) => {
                        panic!("line {}: {}: should have an entry already", i+1, line);
                    }
                }
            },
            [size, _] => {
                match fs.entry(current_dir.clone()) {
                    Entry::Occupied(ref mut o) => {
                        o.get_mut().content.push(FSEntry::File(size.parse::<u32>().unwrap()));
                    },
                    Entry::Vacant(_) => {
                        panic!("line {}: {}: should have an entry already", i+1, line);
                    }
                }
            },
            _ => panic!("invalid log line"),
        }
    }
    Ok(fs)
}

fn compute_dir_size(fs: &HashMap<Vec<String>, Dir>,  dir: &[String]) -> u32 {
    let dir_content = &fs.get(dir).unwrap().content;
    let mut dir_size = 0;
    for fse in dir_content {
        match fse {
            FSEntry::Dir(d) =>  {
                let mut pth = dir.to_vec();
                pth.push(d.clone());
                dir_size += compute_dir_size(fs, &pth);
            },
            FSEntry::File(size) => {
                dir_size += size;
            },
        }
    }
    dir_size
}

fn ex1(fs: &HashMap<Vec<String>, Dir>) -> u32 {
    let mut total = 0;
    for path in fs.keys() {
        if path.len() <= 1 {
            continue;
        }
        let dir_size = compute_dir_size(fs, path);
        if dir_size <= 100_000 {
            total += dir_size;
        }
    }
    total
}

fn ex2(fs: &HashMap<Vec<String>, Dir>) -> u32 {
    let mut min_size = 0;
    let free_space = 70_000_000 - compute_dir_size(fs, &["/".to_string()]);
    for path in fs.keys() {
        let dir_size = compute_dir_size(fs, path);
        let potential_free = dir_size + free_space;
        if potential_free >= 30_000_000 && (min_size == 0 || min_size > potential_free) {
            min_size = dir_size;
        }
    }
    min_size
}

fn main() {
    let fs = build_filesystem("inputs/d7_1.txt").unwrap();
    println!("ex1: {}", ex1(&fs));
    println!("ex2: {}", ex2(&fs));
}
