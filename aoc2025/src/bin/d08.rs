use std::collections::{HashMap, HashSet};

use aoc2025::run_day;

use anyhow::{Context, Result};
use itertools::Itertools;
use union_find::{QuickUnionUf, UnionByRank, UnionFind};

const DAY: &str = "d08";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn dist(&self, other: &Self) -> f64 {
        f64::sqrt(
            (i64::pow(self.x - other.x, 2)
                + i64::pow(self.y - other.y, 2)
                + i64::pow(self.z - other.z, 2)) as f64,
        )
    }
}

impl From<(i64, i64, i64)> for JunctionBox {
    fn from(value: (i64, i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

fn parse_input(file: &str) -> Result<Vec<JunctionBox>> {
    file.lines()
        .map(|l| -> Option<JunctionBox> {
            l.split(',')
                .filter_map(|e| e.parse::<i64>().ok())
                .collect_tuple::<(i64, i64, i64)>()
                .map(Into::into)
        })
        .collect::<Option<Vec<_>>>()
        .context("failed to parse file")
}

fn compute_distances(boxes: &[JunctionBox]) -> Vec<((JunctionBox, JunctionBox), f64)> {
    let distances_map: HashMap<(JunctionBox, JunctionBox), f64> = boxes
        .iter()
        .flat_map(|a| boxes.iter().map(|b| ((*a).clone(), (*b).clone())))
        .filter(|(a, b)| !a.eq(b))
        .map(move |(a, b)| {
            let d = a.dist(&b);
            if a < b { ((a, b), d) } else { ((b, a), d) }
        })
        .collect();

    let mut distances: Vec<_> = distances_map.into_iter().collect();
    distances.sort_by(|a, b| a.1.total_cmp(&b.1));
    distances
}

fn compute_groups(distances: &[((JunctionBox, JunctionBox), f64)]) -> Vec<HashSet<JunctionBox>> {
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();

    for (pair, _) in distances.iter() {
        let mut hs = HashSet::new();
        hs.insert(pair.0.to_owned());
        hs.insert(pair.1.to_owned());
        circuits.push(hs);
    }

    let mut uf = QuickUnionUf::<UnionByRank>::new(circuits.len());
    for (i, s1) in circuits.iter().enumerate() {
        for (j, s2) in circuits.iter().enumerate() {
            if !s1.is_disjoint(s2) {
                uf.union(i, j);
            }
        }
    }

    let mut groups = vec![HashSet::new(); circuits.len()];
    for (i, c) in circuits.iter().enumerate() {
        let root = uf.find(i);
        groups[root].extend(c.clone());
    }

    let mut groups: Vec<HashSet<_>> = groups.into_iter().filter(|s| !s.is_empty()).collect();
    groups.sort_by_key(|a| std::cmp::Reverse(a.len()));
    groups
}

fn ex1_implem(boxes: &[JunctionBox], num_circuits_to_count: usize) -> Result<i64> {
    let distances = compute_distances(boxes);

    let groups = compute_groups(&distances[..num_circuits_to_count]);

    Ok(groups.iter().take(3).map(|g| g.len()).product::<usize>() as i64)
}

fn ex1(file: &str) -> Result<i64> {
    ex1_implem(parse_input(file)?.as_slice(), 1000)
}

fn divide_to_conquer(
    distances: &[((JunctionBox, JunctionBox), f64)],
    boxes_len: usize,
    jump_size: usize,
    start: usize,
) -> (usize, i64) {
    let mut i = start;
    loop {
        i += jump_size;
        let groups = compute_groups(&distances[..i]);
        if groups.len() == 1 && groups[0].len() == boxes_len {
            println!("found in {} window", jump_size);
            let last_conn = &distances[i - 1];
            return (i, last_conn.0.0.x * last_conn.0.1.x);
        }
    }
}

fn ex2_implem(boxes: &[JunctionBox], start_num: usize) -> Result<i64> {
    let distances = compute_distances(boxes);

    let mut start = start_num;
    let mut jump_size = start_num;

    loop {
        // Binary search would be better for worst-case scenario, but this is no such scenario :D
        let (i, res) = divide_to_conquer(&distances, boxes.len(), jump_size, start);
        if jump_size == 1 {
            return Ok(res);
        }
        start = i - jump_size;
        jump_size /= 10;
    }
}

fn ex2(file: &str) -> Result<i64> {
    let boxes = parse_input(file)?;
    ex2_implem(boxes.as_slice(), 1000)
}

fn main() {
    run_day(DAY, ex1, ex2);
}

#[cfg(test)]
mod tests {
    use crate::{ex1_implem, ex2_implem, parse_input};

    use super::DAY;
    use std::fs;

    use super::{ex1, ex2};

    #[test]
    fn test() {
        let example = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        let expected_ex1: i64 = 40;
        let expected_ex2: i64 = 25272;
        assert_eq!(
            expected_ex1,
            ex1_implem(parse_input(example).unwrap().as_slice(), 10).expect("ex1 failed")
        );
        assert_eq!(
            expected_ex2,
            ex2_implem(parse_input(example).unwrap().as_slice(), 10).expect("ex2 failed")
        );
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 68112;
        let expected_ex2: i64 = 44543856;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
