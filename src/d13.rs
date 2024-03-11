use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

enum Item {
    List(Vec<Item>),
    Integer(u32),
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars().peekable();
        let mut res = Vec::new();
        loop {
            let next_elem = match s.peek() {
                Some('[') => s
                    .by_ref()
                    .take_while_inclusive(|char| *char != ']')
                    .collect::<String>()
                    .parse::<Item>()?,
                _ => {
                    let num = s
                        .by_ref()
                        .take_while(|char| *char != ',')
                        .collect::<String>()
                        .parse::<u32>()?;
                    Item::Integer(num)
                }
            };
            res.push(next_elem);
            match s.next() {
                Some(',') => continue,
                Some(']') => break Ok(Item::List(res)),
                _ => unreachable!(),
            };
        }
    }
}

pub fn p1(file: &str) -> Result<u32> {
    for pair in file.split("\n\n") {
        let (first, second) = pair.split_once('\n').unwrap();
        first.parse::<Item>()?;
        second.parse::<Item>()?;
    }
    todo!()
}
pub fn p2(file: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d13/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 21);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d13/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d13/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 8);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d13/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
