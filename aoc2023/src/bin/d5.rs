use core::str::Lines;
use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};
use itertools::Itertools;
use rangemap::RangeInclusiveMap;

fn fill_rangemap(mut lines: Lines<'_>) -> Result<RangeInclusiveMap<u64, u64>> {
    let mut m = RangeInclusiveMap::new();
    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let l = l.unwrap();
        if l.is_empty() {
            break;
        }
        let mut vals = l.split_whitespace();
        let value = vals
            .next()
            .ok_or_else(|| Error::msg("bad map line"))?
            .parse::<u64>()?;
        let start = vals
            .next()
            .ok_or_else(|| Error::msg("bad map line"))?
            .parse::<u64>()?;
        let end = start
            + vals
                .next()
                .ok_or_else(|| Error::msg("bad map line"))?
                .parse::<u64>()?
            - 1;
        m.insert(start..=end, value)
    }
    Ok(m)
}

fn fill_maps(mut lines: Lines<'_>, maps: &mut [RangeInclusiveMap<u64, u64>; 7]) -> Result<()> {
    let mut index = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let l = l.unwrap();
        if l.is_empty() {
            continue;
        }
        if l.ends_with(':') {
            maps[index] = fill_rangemap(lines.clone())?;
            index += 1;
        }
    }
    Ok(())
}

fn ex1(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let seeds = lines.next().ok_or_else(|| Error::msg("empty file"))?[7..].split_whitespace();
    let mut maps: [RangeInclusiveMap<u64, u64>; 7] = Default::default();
    fill_maps(lines, &mut maps)?;

    let mut min = u64::MAX;
    for s in seeds {
        let mut v = s.parse::<u64>()?;
        for m in maps.iter() {
            if let Some((k, val)) = m.get_key_value(&v) {
                v = *val + (v - k.start());
            }
        }
        if v < min {
            min = v;
        }
    }
    Ok(min)
}

fn ex2(file: &str) -> Result<u64> {
    let mut lines = file.lines();
    let seeds = lines.next().ok_or_else(|| Error::msg("empty file"))?[7..].split_whitespace();
    let mut maps: [RangeInclusiveMap<u64, u64>; 7] = Default::default();
    fill_maps(lines, &mut maps)?;

    let mut min = u64::MAX;
    for (start, count) in seeds.tuples() {
        let start = start.parse::<u64>()?;
        let end = start + count.parse::<u64>()? - 1;
        let mut seed = start;
        while seed <= end {
            let mut v = seed;
            let mut smallest_intersection = u64::MAX;
            for m in maps.iter() {
                if let Some((k, val)) = m.get_key_value(&v) {
                    let range_size = k.end()+1-k.start();
                    if range_size < smallest_intersection {
                        // find the smallest range containing seed
                        // all next seed values until the end of that shortest range
                        // will be the result for this seed value + 1, 2, 3 etc.
                        // so they can be skipped
                        smallest_intersection = k.end()+1-v;
                    }
                    v = *val + (v - k.start());
                }
            }
            if v < min {
                min = v;
            }
            
            seed += if smallest_intersection != u64::MAX {
                smallest_intersection
            } else {
                1
            }
        }
    }
    Ok(min)
}

fn main() {
    let file = fs::read_to_string("./inputs/d5_1.txt").unwrap();

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
    use super::*;
    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, ex1(input).unwrap());
        assert_eq!(46, ex2(input).unwrap());
    }
}
