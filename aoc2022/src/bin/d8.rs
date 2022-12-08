use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

static RADIX: u32 = 10;

fn parse_map(filename: &str) -> io::Result<Vec<Vec<i8>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut res = Vec::new();
    for line in reader.lines() {
        res.push(line?.chars().map(|c| c.to_digit(RADIX).unwrap() as i8).collect::<Vec<i8>>());
    }
    Ok(res)
}

fn tree_index(i: usize, j: usize, map_dim: usize) -> usize {
    i*map_dim+j
}

fn compute_tree_scenic_score(map: &[Vec<i8>], tree_x: usize, tree_y: usize, map_dim: usize) -> usize {
    let visibility_lvl = map[tree_x][tree_y];
    let mut directions = [0; 4];
    let mut score = 0;
    for i in (0..tree_x).rev() {
        score += 1;
        if map[i][tree_y] >= visibility_lvl {
            break;
        }
    }
    directions[0] = score;
    score = 0;
    for j in (0..tree_y).rev() {
        score += 1;
        if map[tree_x][j] >= visibility_lvl {
            break;
        }
    }
    directions[1] = score;
    score = 0;
    for i in tree_x+1..map_dim {
        score += 1;
        if map[i][tree_y] >= visibility_lvl {
            break;
        }
    }
    directions[2] = score;
    score = 0;
    for j in tree_y+1..map_dim {
        score += 1;
        if map[tree_x][j] >= visibility_lvl {
            break;
        }
    }
    directions[3] = score;
    directions.iter().product()
}

fn ex2(map: &[Vec<i8>]) -> usize {
    let map_dim = map.len();
    let mut max_score = 0;
    for i in 0..map_dim {
        for j in 0..map_dim {
            let score = compute_tree_scenic_score(map, i, j, map_dim);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn compute_outside_visibility(map: &[Vec<i8>]) -> usize {
    let mut visible_trees = HashSet::new();
    let map_dim = map.len();
    for col_index in 0..map_dim {
        let mut visibility_lvl = -1i8;
        let mut rev_visibility_lvl = -1i8;
        for row_index in 0..map_dim {
            let cell = map[col_index][row_index];
            if cell > visibility_lvl {
                visible_trees.insert(tree_index(col_index, row_index, map_dim));
                visibility_lvl = cell;
            }
            let rev_index = map_dim-1-row_index;
            let cell = map[col_index][rev_index];
            if cell > rev_visibility_lvl {
                visible_trees.insert(tree_index(col_index, rev_index , map_dim));
                rev_visibility_lvl = cell;
            }
            if visibility_lvl == 9 && rev_visibility_lvl == 9 {
                break;
            }
        }
    }
    for row_index in 0..map_dim {
        let mut visibility_lvl = -1i8;
        let mut rev_visibility_lvl = -1i8;
        for col_index in 0..map_dim {
            let cell = map[col_index][row_index];
            if cell > visibility_lvl {
                visible_trees.insert(tree_index(col_index, row_index, map_dim));
                visibility_lvl = cell;
            }
            let rev_index = map_dim-1-col_index;
            let cell = map[rev_index][row_index];
            if cell > rev_visibility_lvl {
                visible_trees.insert(tree_index(rev_index, row_index , map_dim));
                rev_visibility_lvl = cell;
            }
            if visibility_lvl == 9 && rev_visibility_lvl == 9 {
                break;
            }
        }
    }

    visible_trees.len()
}

fn ex1(map: &[Vec<i8>]) -> usize {
    compute_outside_visibility(map)
}

fn main() {
    let map = parse_map("inputs/d8_1.txt").unwrap();
    println!("ex1: {}", ex1(&map));
    println!("ex2: {}", ex2(&map));
}