use std::{borrow::BorrowMut, cmp::Ordering, fmt::Display, str::FromStr};

use anyhow::{Context, Error, Result};
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Integer(u32),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(list) => write!(f, "{list:?}"),
            Item::Integer(num) => write!(f, "{num}"),
        }
    }
}

impl FromStr for Item {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iter = s.chars().peekable();

        match s_iter.peek() {
            Some('[') => {
                s_iter.next();

                let mut items = Vec::new();
                let mut buf = String::new();
                let mut unclosed_brackets = 0;

                for next in s_iter {
                    if unclosed_brackets == 0 && (next == ',' || next == ']') {
                        if buf.is_empty() {
                            break;
                        }
                        let item = buf.parse::<Item>()?;
                        items.push(item);
                        buf.clear();
                    } else {
                        match next {
                            '[' => unclosed_brackets += 1,
                            ']' => unclosed_brackets -= 1,
                            _ => (),
                        }
                        buf.push(next);
                    }
                }
                Ok(Item::List(items))
            }
            Some(_part_of_a_num) => Ok(Item::Integer(
                s_iter
                    .borrow_mut()
                    .take_while(|char| *char != ',')
                    .collect::<String>()
                    .parse::<u32>()?,
            )),
            None => unreachable!(),
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
                for pair in left_list.iter().zip_longest(right_list) {
                    match pair {
                        EitherOrBoth::Right(_) => return Some(Ordering::Less),
                        EitherOrBoth::Left(_) => return Some(Ordering::Greater),
                        EitherOrBoth::Both(left_item, right_item) => {
                            match left_item.partial_cmp(right_item) {
                                Some(Ordering::Greater) => return Some(Ordering::Greater),
                                Some(Ordering::Less) => {
                                    smaller = true;
                                }
                                Some(Ordering::Equal) => continue,
                                None => unreachable!(),
                            }
                        }
                    }
                }
                if smaller {
                    return Some(Ordering::Less);
                }
                Some(Ordering::Equal)
            }
            (Self::List(list), Self::Integer(num)) => list.partial_cmp(&vec![Self::Integer(*num)]),
            (Self::Integer(num), Self::List(list)) => vec![Self::Integer(*num)].partial_cmp(list),
        }
    }
}

pub fn p1(file: &str) -> Result<usize> {
    let mut sum = 0;
    for (idx, pair) in file.split("\n\n").enumerate() {
        let (first, second) = pair
            .split_once('\n')
            .context("Couldn't split left and right")?;
        let first = first.parse::<Item>()?;
        let second = second.parse::<Item>()?;
        if first < second {
            sum += idx + 1;
        } else {
            println!("{first} > {second}")
        }
    }
    Ok(sum)
}
pub fn p2(_file: &str) -> Result<u32> {
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
    #[test_case(L(vec![L(vec![I(1)]),L(vec![I(2),I(3),I(4)])]), L(vec![L(vec![I(1)]),I(4)]))]
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
    #[test_case(L(vec![L(vec![L(vec![])])]), L(vec![L(vec![])]))]
    fn larger(l0: Item, r0: Item) {
        assert!(l0 > r0);
    }

    #[test_case("[3,4]", L(vec![I(3),I(4)]))]
    #[test_case("[]", L(vec![]))]
    #[test_case("[[3],[]]", L(vec![L(vec![I(3)]), L(vec![])]))]
    #[test_case("[[],[[]],[1]]", L(vec![L(vec![]), L(vec![L(vec![])]), L(vec![I(1)])]))]
    #[test_case("[[[[]]],[]]", L(vec![L(vec![L(vec![L(vec![])])]), L(vec![])]))]
    #[test_case("[[1],[2,3,4]]", L(vec![L(vec![I(1)]),L(vec![I(2),I(3),I(4)])]))]
    #[test_case("[[1],4]", L(vec![L(vec![I(1)]),I(4)]))]
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
