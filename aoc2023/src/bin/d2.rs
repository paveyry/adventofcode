use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

fn ex1(file: &str) -> Result<u32> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let mut sum = 0;

    for l in file.lines() {
        let mut s = l.split(": ");
        let game_id_iter = s
            .next()
            .ok_or_else(|| Error::msg("failed to get game id"))?;

        let sets = s
            .next()
            .ok_or_else(|| Error::msg("missing sets info"))?
            .split("; ");

        let mut set_valid = true;

        for s in sets {
            for e in s.split(", ") {
                let mut entry_info = e.split(' ');
                let num = entry_info
                    .next()
                    .ok_or_else(|| Error::msg("failed to get color num"))?
                    .parse::<u32>()?;
                let color = entry_info
                    .next()
                    .ok_or_else(|| Error::msg("failed to get color name"))?;
                let invalid = match color {
                    "blue" => num > MAX_BLUE,
                    "green" => num > MAX_GREEN,
                    "red" => num > MAX_RED,
                    _ => {
                        return Err(Error::msg("invalid color name"));
                    }
                };
                if invalid {
                    set_valid = false;
                }
            }
        }
        if set_valid {
            let game_id = game_id_iter
                .split(' ')
                .nth(1)
                .ok_or_else(|| Error::msg("failed to get game id num"))?
                .parse::<u32>()?;
            sum += game_id;
        }
    }

    Ok(sum)
}

fn ex2(file: &str) -> Result<u32> {
    let mut sum = 0;

    for l in file.lines() {
        let sets = l
            .split(": ")
            .nth(1)
            .ok_or_else(|| Error::msg("missing sets info"))?
            .split("; ");

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for s in sets {
            for e in s.split(", ") {
                let mut entry_info = e.split(' ');
                let num = entry_info
                    .next()
                    .ok_or_else(|| Error::msg("failed to get color num"))?
                    .parse::<u32>()?;
                let color = entry_info
                    .next()
                    .ok_or_else(|| Error::msg("failed to get color name"))?;
                match color {
                    "blue" => {
                        if num > max_blue {
                            max_blue = num;
                        }
                    }
                    "green" => {
                        if num > max_green {
                            max_green = num;
                        }
                    }
                    "red" => {
                        if num > max_red {
                            max_red = num;
                        }
                    }
                    _ => {
                        return Err(Error::msg("invalid color name"));
                    }
                };
            }
        }
        sum += max_blue * max_green * max_red;
    }

    Ok(sum)
}

fn main() {
    let file = fs::read_to_string("./inputs/d2_1.txt").unwrap();

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
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, ex1(example).unwrap());
        assert_eq!(2286, ex2(example).unwrap());
    }
}
