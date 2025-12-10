use std::str::FromStr;

use aoc2025::run_day;

use anyhow::{Context, Result};

const DAY: &str = "d10";

#[derive(Clone, Default)]
struct Machine {
    goal: u16,
    buttons_masks: Vec<u16>,
    joltage_req: Vec<i64>,
}

impl Machine {
    fn find_goal_in_n_presses(&self, start: u16, presses: i64) -> bool {
        if start == self.goal {
            return true;
        }
        if presses == 0 {
            return false;
        }
        for b in self.buttons_masks.iter() {
            let modified = start ^ b;
            if self.find_goal_in_n_presses(modified, presses - 1) {
                return true;
            }
        }
        false
    }

    fn find_best_combinations_for_joltage(&self) -> Option<i64> {
        let optimizer = z3::Optimize::new();
        let z3_buttons = (0..self.buttons_masks.len())
            .map(|i| i as u32)
            .map(z3::ast::Int::new_const)
            .collect::<Vec<_>>();
        z3_buttons.iter().for_each(|z3b| {
            optimizer.assert(&z3b.ge(z3::ast::Int::from_i64(0)));
        });
        for (index, joltage) in self.joltage_req.iter().enumerate() {
            let affecting_buttons = self
                .buttons_masks
                .iter()
                .enumerate()
                .filter_map(|(b_index, b)| {
                    if b & (1 << index) != 0 {
                        Some(z3_buttons[b_index].clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if affecting_buttons.is_empty() && self.goal != 0 {
                return None;
            }
            let s = affecting_buttons.into_iter().reduce(|a, b| a + b)?;
            if s != 0 {
                optimizer.assert(&s.eq(z3::ast::Int::from_i64(*joltage)));
            }
        }

        let num_presses = z3_buttons
            .iter()
            .map(ToOwned::to_owned)
            .reduce(|a, b| a + b)?;
        optimizer.minimize(&num_presses);

        if optimizer.check(&[]) == z3::SatResult::Sat {
            optimizer.get_model()?.eval(&num_presses, true)?.as_i64()
        } else {
            None
        }
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let closing_brack = s.find("]").context("failed to find closing bracket")?;
        let mut goal = 0;
        (s[1..closing_brack])
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                if c == '#' {
                    goal |= 1 << i;
                }
            });
        let opening_curly = s.find("{").context("failed to find opening curly")?;
        let buttons_masks = (s[closing_brack + 3..opening_curly - 2])
            .split(") (")
            .map(|b| {
                let mut bitset = 0;
                b.split(',').for_each(|c| {
                    if let Ok(index) = c.parse::<usize>() {
                        bitset |= 1 << index;
                    }
                });
                Ok(bitset)
            })
            .collect::<Result<_>>()?;
        let joltage_req = (s[opening_curly + 1..s.len() - 1])
            .split(',')
            .map(|s| Ok(s.parse()?))
            .collect::<Result<_>>()?;

        Ok(Machine {
            goal,
            buttons_masks,
            joltage_req,
        })
    }
}

fn ex1(file: &str) -> Result<i64> {
    let mut sum = 0;
    let machines = file
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()?;
    for m in machines.iter() {
        let mut i = 1;
        while !m.find_goal_in_n_presses(0, i) {
            i += 1;
        }
        sum += i;
    }
    Ok(sum)
}

fn ex2(file: &str) -> Result<i64> {
    let machines = file
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()?;
    let sum = machines
        .iter()
        .filter_map(|m| m.find_best_combinations_for_joltage())
        .sum::<i64>();

    Ok(sum)
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
        let example = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        let expected_ex1: i64 = 7;
        let expected_ex2: i64 = 33;
        assert_eq!(expected_ex1, ex1(example).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(example).expect("ex2 failed"));
    }

    #[test]
    fn test_file() {
        let file =
            fs::read_to_string(format!("./inputs/{DAY}_1.txt")).expect("failed to read input file");
        let expected_ex1: i64 = 422;
        let expected_ex2: i64 = 16361;
        assert_eq!(expected_ex1, ex1(&file).expect("ex1 failed"));
        assert_eq!(expected_ex2, ex2(&file).expect("ex2 failed"));
    }
}
