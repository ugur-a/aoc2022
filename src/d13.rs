use std::{borrow::BorrowMut, cmp::Ordering, str::FromStr};

use anyhow::{Context, Error, Result};
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Integer(u32),
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        let mut s_iter = s
            .strip_prefix('[')
            .context("invalid list formatting: no opening bracket")?
            .chars()
            .peekable();
        loop {
            let item = match s_iter.peek() {
                Some('[') => {
                    let list = s_iter
                        .borrow_mut()
                        .take_while(|char| *char != ']')
                    .collect::<String>()
                        .parse::<Item>()?;
                    s_iter.next();
                    list
                }
                Some(']') | None => break Ok(Item::List(items)),
                Some(',') => {
                    s_iter.next();
                    continue;
                }
                Some(_part_of_num) => {
                    let num = s_iter
                        .borrow_mut()
                        .take_while(|char| *char != ',' && *char != ']')
                        .collect::<String>()
                        .parse::<u32>()?;
                    Item::Integer(num)
                }
            };
            items.push(item);

            if let None = s_iter.peek() {
                break Ok(Item::List(items));
            } else {
                continue;
            };
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(list), Self::Integer(num)) | (Self::Integer(num), Self::List(list)) => {
                *list == vec![Self::Integer(*num)]
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::List(left_list), Self::List(right_list)) => {
                let mut smaller = false;
                let mut zip = left_list.iter().zip_longest(right_list);
                loop {
                    match zip.next() {
                        Some(EitherOrBoth::Right(_)) => break Some(Ordering::Less),
                        Some(EitherOrBoth::Left(_)) => break Some(Ordering::Greater),
                        Some(EitherOrBoth::Both(left_num, right_num)) => {
                            match left_num.partial_cmp(right_num) {
                                Some(Ordering::Greater) => break Some(Ordering::Greater),
                                Some(Ordering::Less) => {
                                    smaller = true;
                                }
                                Some(Ordering::Equal) => continue,
                                None => unreachable!(),
                            }
                        }
                        None => {
                            if smaller {
                                break Some(Ordering::Less);
                            }
                            break Some(Ordering::Equal);
                        }
                    }
                }
            }
            (Self::List(list), Self::Integer(num)) => list.partial_cmp(&vec![Self::Integer(*num)]),
            (Self::Integer(num), Self::List(list)) => vec![Self::Integer(*num)].partial_cmp(list),
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
    use test_case::test_case;
    use Item::{Integer as I, List as L};

    #[test_case(I(5), I(6))]
    #[test_case(I(5), L(vec![I(6)]); "Integer VS List")]
    #[test_case(L(vec![I(5)]), L(vec![I(6)]))]
    #[test_case(L(vec![I(5)]), L(vec![I(6), I(0)]); "Left list shorter")]
    fn less(l0: Item, r0: Item) {
        assert!(l0 < r0);
    }

    #[test_case(I(5), I(5))]
    #[test_case(L(vec![I(5)]), I(5); "Integer VS List")]
    #[test_case(L(vec![I(5), I(6)]), L(vec![I(5), I(6)]))]
    fn equal(l0: Item, r0: Item) {
        assert!(l0 == r0);
    }

    #[test_case(I(6), I(5))]
    #[test_case(L(vec![I(6), I(6)]), L(vec![I(5), I(6)]))]
    #[test_case(L(vec![I(5), I(6)]), L(vec![I(5)]); "Left list longer")]
    fn larger(l0: Item, r0: Item) {
        assert!(l0 > r0);
    }

    #[test_case("[]", L(vec![]))]
    #[test_case("[[3],[]]", L(vec![L(vec![I(3)]), L(vec![])]))]
    #[test_case("[[],[[]],[1]]", L(vec![L(vec![]), L(vec![L(vec![])]), L(vec![I(1)])]))]
    fn parse_item(s: &str, item: Item) {
        assert_eq!(s.parse::<Item>().unwrap(), item)
    }

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
