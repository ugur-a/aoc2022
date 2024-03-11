use std::{borrow::BorrowMut, cmp::Ordering, str::FromStr};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Integer(u32),
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        let mut s = s.chars().skip(1).peekable();
        loop {
            let item = match s.peek() {
                Some('[') => {
                    let list = s
                        .borrow_mut()
                        .take_while(|char| *char != ']')
                    .collect::<String>()
                        .parse::<Item>()?;
                    s.next();
                    list
                }
                Some(_part_of_num) => {
                    let num = s
                        .borrow_mut()
                        .take_while(|char| *char != ',' && *char != ']')
                        .collect::<String>()
                        .parse::<u32>()?;
                    Item::Integer(num)
                }
                _ => unreachable!(),
            };
            items.push(item);

            if let None = s.peek() {
                break Ok(Item::List(items));
            } else {
                continue;
            };
        }
    }
}

        }
    }
}

pub fn p1(file: &str) -> Result<u32> {
    for pair in file.split("\n\n") {
        let (first, second) = pair.split_once('\n').unwrap();
        let first = first.parse::<Item>()?;
        let second = second.parse::<Item>()?;
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
        assert_eq!(p1(&inp).unwrap(), 13);
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
