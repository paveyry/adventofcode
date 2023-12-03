use std::cmp::{max, min};
use std::fs;
use std::collections::{HashSet, HashMap};
use std::time::Instant;

use anyhow::{Error, Result};

fn has_neighbour_sym(arr: &Vec<&str>, x: usize, y: usize) -> bool {
    let min_x = max(x as isize - 1, 0);
    let min_y = max(y as isize - 1, 0);
    let max_x = min(x as isize + 1, arr.len() as isize - 1);
    let max_y = min(y as isize + 1, arr.get(0).unwrap().len() as isize - 1);
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            let c = arr
                .get(i as usize)
                .unwrap()
                .chars()
                .nth(j as usize)
                .unwrap();
            if !c.is_numeric() && c != '.' {
                return true;
            }
        }
    }
    false
}

fn ex1(file: &str) -> Result<u32> {
    let arr: Vec<&str> = file.lines().collect();
    let mut sum = 0;
    for (i, line) in file.lines().enumerate() {
        let mut current_num = 0u32;
        let mut current_num_size = 0usize;
        let mut count_current_num = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_num_size += 1;
                current_num = current_num * 10
                    + c.to_digit(10)
                        .ok_or_else(|| Error::msg("failed to convert char to digit"))?;
                if has_neighbour_sym(&arr, i, j) {
                    count_current_num = true;
                }
                if j < line.len() - 1 {
                    continue;
                }
            }
            if current_num_size == 0 {
                continue;
            }
            if count_current_num {
                sum += current_num;
            }
            current_num = 0;
            current_num_size = 0;
            count_current_num = false;
        }
    }
    Ok(sum)
}

fn count_neighbour_nums(nums_map: &HashMap<(usize,usize), u32>, x: usize, y: usize, neighbours: &mut HashSet<u32>) {
    let min_x = max(x as isize - 1, 0) as usize;
    let min_y = max(y as isize - 1, 0) as usize;
    for i in min_x..=x+1 {
        for j in min_y..=y+1 {
            if let Some(n) = nums_map.get(&(i, j)) {
                neighbours.insert(*n);
            }
        }
    }
}

fn ex2(file: &str) -> Result<u32> {
    let mut nums_map: HashMap<(usize,usize), u32> = HashMap::new();
    let mut nums_values_map: HashMap<u32, u32> = HashMap::new();
    let mut syms_set: HashSet<(usize,usize)> = HashSet::new();
    let mut cur_num_id = 0;
    for (i, line) in file.lines().enumerate() {
        let mut current_num = 0u32;
        let mut current_num_size = 0usize;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_num_size += 1;
                current_num = current_num * 10
                    + c.to_digit(10)
                        .ok_or_else(|| Error::msg("failed to convert char to digit"))?;
                nums_map.insert((i, j), cur_num_id);
                if j < line.len() - 1 {
                    continue;
                }
            } else if c != '.' {
                syms_set.insert((i, j));
            }
            if current_num_size == 0 {
                continue;
            }
            nums_values_map.insert(cur_num_id, current_num);
            current_num = 0;
            cur_num_id += 1;
            current_num_size = 0;
        }
    }
    let mut sum = 0;
    let mut neighbours = HashSet::new();
    for (i, j) in syms_set {
        neighbours.clear();
        count_neighbour_nums(&nums_map, i, j, &mut neighbours);
        if neighbours.len() == 2 {
            sum += neighbours.iter().map(|n| nums_values_map.get(n).unwrap()).product::<u32>();
        }
    }
    Ok(sum)
}

fn main() {
    let file = fs::read_to_string("./inputs/d3_1.txt").unwrap();

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
mod tests {
    use super::{ex1, ex2};

    #[test]
    fn test() {
        let example = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, ex1(example).unwrap());
        assert_eq!(467835, ex2(example).unwrap());
    }
}
