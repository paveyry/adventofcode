use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::time::Instant;

use anyhow::{Error, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType<const T: bool> {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a, const T: bool> {
    typ: HandType<T>,
    cards: &'a str,
}

impl<'a, const T: bool> PartialOrd for Hand<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.typ != other.typ {
            return self.typ.partial_cmp(&other.typ);
        }
        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            let mut cv1 = card_name_to_value(c1).unwrap();
            let mut cv2 = card_name_to_value(c2).unwrap();
            if T {
                cv1 = card_name_value_with_joker(cv1);
                cv2 = card_name_value_with_joker(cv2);
            }
            match cv1.cmp(&cv2) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => {
                    return Some(Ordering::Less);
                }
                Ordering::Equal => {}
            }
        }
        Some(Ordering::Equal)
    }
}

impl<'a, const T: bool> Ord for Hand<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<const T: bool> TryFrom<&str> for HandType<T> {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut hm: HashMap<char, u32> = HashMap::new();
        let mut joker_count = 0;
        s.chars().for_each(|c| {
            if T && c == 'J' {
                joker_count += 1;
            } else {
                *hm.entry(c).or_default() += 1;
            }
        });
        if joker_count >= 4 {
            return Ok(HandType::FiveOfAKind);
        }
        match hm.len() {
            1 => Ok(HandType::FiveOfAKind),
            5 => Ok(HandType::HighCard),
            _ => {
                let mut lengths = hm.values().copied().collect::<Vec<_>>();
                lengths.sort();
                if let Some(v) = lengths.last_mut() {
                    *v += joker_count;
                };
                match lengths[..] {
                    [1, 4] => Ok(HandType::FourOfAKind),
                    [2, 3] => Ok(HandType::FullHouse),
                    [1, 1, 3] => Ok(HandType::ThreeOfAKind),
                    [1, 2, 2] => Ok(HandType::TwoPair),
                    [1, 1, 1, 2] => Ok(HandType::OnePair),
                    _ => Err(Error::msg(format!(
                        "error computing hand type: {} {:?}",
                        s, lengths
                    ))),
                }
            }
        }
    }
}

fn card_name_to_value(c: char) -> Result<u8> {
    match c {
        'T' => Ok(10),
        'J' => Ok(11),
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        _ => {
            let n = c
                .to_digit(10)
                .ok_or_else(|| Error::msg("invalid card name"))?;
            match n {
                2..=9 => Ok(n as u8),
                _ => Err(Error::msg("invalid card name")),
            }
        }
    }
}

fn card_name_value_with_joker(v: u8) -> u8 {
    if v == 11 {
        return 1;
    }
    v
}

fn ex1(file: &str) -> Result<u64> {
    let mut m = BTreeMap::new();
    for l in file.lines() {
        let (hand_str, bid_str) = l.split_at(5);
        m.insert(
            Hand::<false> {
                typ: hand_str.try_into()?,
                cards: hand_str,
            },
            bid_str.trim().parse::<u64>()?,
        );
    }
    Ok(m.values()
        .enumerate()
        .map(|(i, bid)| *bid * (i + 1) as u64)
        .sum())
}

fn ex2(file: &str) -> Result<u64> {
    let mut m = BTreeMap::new();
    for l in file.lines() {
        let (hand_str, bid_str) = l.split_at(5);
        m.insert(
            Hand::<true> {
                typ: hand_str.try_into()?,
                cards: hand_str,
            },
            bid_str.trim().parse::<u64>()?,
        );
    }
    Ok(m.values()
        .enumerate()
        .map(|(i, bid)| *bid * (i + 1) as u64)
        .sum())
}

fn main() {
    let file = fs::read_to_string("./inputs/d7_1.txt").unwrap();

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
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, ex1(input).unwrap());
        assert_eq!(5905, ex2(input).unwrap());
    }

    #[test]
    fn test_file() {
        let file = fs::read_to_string("./inputs/d7_1.txt").unwrap();
        assert_eq!(249726565, ex1(&file).unwrap());
        assert_eq!(251135960, ex2(&file).unwrap());
    }
}
