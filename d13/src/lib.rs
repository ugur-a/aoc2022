use aoc2022lib::impl_from_str_from_nom_parser;
use itertools::Itertools;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use nom::{
    branch::alt,
    character::complete::{char, u8},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Eq)]
enum Item {
    List(Vec<Item>),
    Integer(u8),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(list) => write!(f, "{list:?}"),
            Item::Integer(num) => write!(f, "{num}"),
        }
    }
}

fn integer(input: &str) -> IResult<&str, Item> {
    map(u8, Item::Integer)(input)
}

fn list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(char('['), separated_list0(char(','), item), char(']')),
        Item::List,
    )(input)
}

// [[[[]]],[]]
// [[1],[2,3,4]]
fn item(input: &str) -> IResult<&str, Item> {
    alt((integer, list))(input)
}

impl_from_str_from_nom_parser!(item, Item);

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::List(list), Self::Integer(num)) | (Self::Integer(num), Self::List(list)) => {
                *list == vec![Self::Integer(*num)]
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.cmp(r0),
            (Self::List(l0), Self::List(r0)) => l0.cmp(r0),
            (Self::Integer(num), Self::List(list)) => vec![Self::Integer(*num)].cmp(list),
            (Self::List(list), Self::Integer(num)) => list.cmp(&vec![Self::Integer(*num)]),
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let pairs = file
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap().into())
        .map(|pair: [&str; 2]| pair.map(|part| Item::from_str(part).unwrap()));

    let res = (1..)
        .zip(pairs)
        .filter_map(|(idx, [left, right])| (left < right).then_some(idx))
        .sum();

    Ok(res)
}
pub fn p2(file: &str) -> anyhow::Result<usize> {
    let dividers = ["[[2]]", "[[6]]"];

    let mut packets: Vec<Item> = file
        .lines()
        .filter(|line| !line.is_empty())
        .chain(dividers)
        .map(Item::from_str)
        .try_collect()?;
    packets.sort_unstable();

    let res: usize = dividers
        .map(|divider| divider.parse::<Item>().unwrap())
        .map(|divider| packets.binary_search(&divider).unwrap() + 1)
        .iter()
        .product();

    Ok(res)
}

#[allow(clippy::needless_pass_by_value)]
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
        assert_eq!(s.parse::<Item>().unwrap(), item);
    }

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 13);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 5503);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 140);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 20952);
    }
}
