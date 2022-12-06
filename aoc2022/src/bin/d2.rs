use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!(),
        }
    }
}

impl From<Move> for u32 {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn move_from_outcome(elf_move: Move, outcome: &str) -> Move {
    if outcome == "Y" {
        return elf_move;
    }
    match elf_move {
        Move::Rock => {
            if outcome == "X" {
                Move::Scissors
            } else {
                Move::Paper
            }
        }
        Move::Paper => {
            if outcome == "X" {
                Move::Rock
            } else {
                Move::Scissors
            }
        }
        Move::Scissors => {
            if outcome == "X" {
                Move::Paper
            } else {
                Move::Rock
            }
        }
    }
}

fn single_match_score(elf_move: Move, player_move: Move) -> u32 {
    if elf_move == player_move {
        return 3 + u32::from(player_move);
    }
    let score = match elf_move {
        Move::Rock => {
            if let Move::Paper = player_move {
                6
            } else {
                0
            }
        }
        Move::Paper => {
            if let Move::Scissors = player_move {
                6
            } else {
                0
            }
        }
        Move::Scissors => {
            if let Move::Rock = player_move {
                6
            } else {
                0
            }
        }
    };
    score + u32::from(player_move)
}

fn compute_score_ex1(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut total_score = 0;
    for line in reader.lines() {
        let move_strs: Vec<String> = line?.split_whitespace().map(str::to_string).collect();
        let elf_move: Move = move_strs[0].as_str().into();
        let player_move: Move = move_strs[1].as_str().into();
        total_score += single_match_score(elf_move, player_move);
    }
    Ok(total_score)
}

fn compute_score_ex2(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut total_score = 0;
    for line in reader.lines() {
        let move_strs: Vec<String> = line?.split_whitespace().map(str::to_string).collect();
        let elf_move: Move = move_strs[0].as_str().into();
        let outcome = move_strs[1].as_str();
        total_score += single_match_score(elf_move, move_from_outcome(elf_move, outcome));
    }
    Ok(total_score)
}

fn main() {
    println!("ex1: {}", compute_score_ex1("inputs/d2_1.txt").unwrap());
    println!("ex2: {}", compute_score_ex2("inputs/d2_1.txt").unwrap());
}
