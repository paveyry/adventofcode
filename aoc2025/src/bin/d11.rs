use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc2025::run_day;

use anyhow::Result;

const DAY: &str = "d11";

#[derive(Debug, Default, Clone)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn count_paths_memoized(
        &self,
        start: &str,
        out: &str,
        memo: &mut HashMap<(String, String), i64>,
    ) -> i64 {
        let Some(nodes) = self.edges.get(start) else {
            return 0;
        };
        let path = (start.to_owned(), out.to_owned());
        if let Some(&c) = memo.get(&path) {
            return c;
        }

        let count = nodes
            .iter()
            .map(|n| {
                if n == out {
                    1
                } else {
                    self.count_paths_memoized(n, out, memo)
                }
            })
            .sum();
        memo.insert(path, count);
        count
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let edges = s
            .lines()
            .filter_map(|l| {
                let (node, remainder) = l.split_once(": ")?;
                let outputs = remainder
                    .split_whitespace()
                    .map(ToOwned::to_owned)
                    .collect::<HashSet<String>>();
                Some((node.to_owned(), outputs))
            })
            .collect();
        Ok(Self { edges })
    }
}

fn ex1(file: &str) -> Result<i64> {
    let g: Graph = file.parse()?;
    Ok(g.count_paths_memoized("you", "out", &mut HashMap::new()))
}

fn ex2(file: &str) -> Result<i64> {
    let g: Graph = file.parse()?;

    let mut memo = HashMap::new();

    let mut first_stop = "dac";
    let mut second_stop = "fft";
    let mut mid = g.count_paths_memoized(first_stop, second_stop, &mut memo);
    if mid == 0 {
        (first_stop, second_stop) = (second_stop, first_stop);
        mid = g.count_paths_memoized(first_stop, second_stop, &mut memo);
    }
    let start = g.count_paths_memoized("svr", first_stop, &mut memo);
    let end = g.count_paths_memoized(second_stop, "out", &mut memo);
    Ok(start * mid * end)
}

fn main() {
    run_day(DAY, ex1, ex2);
}

#[cfg(test)]
mod tests {
    use super::DAY;
    use std::fs;

    use super::{ex1, ex2};

    #[test]
    fn test() {
        let example = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let expected_ex1: i64 = 5;
        let example2 = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let expected_ex2: i64 = 2;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example2).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 733;
        let expected_ex2: i64 = 290219757077250;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
