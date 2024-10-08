use libaoc::impl_from_str_from_nom_parser;
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
    let div1 = Item::from_str("[[2]]")?;
    let div2 = Item::from_str("[[6]]")?;

    let mut packets: Vec<Item> = file
        .lines()
        .filter(|line| !line.is_empty())
        .map(Item::from_str)
        .try_collect()?;
    packets.sort_unstable();

    // binary_searches' Result:Err is the position where the divider _would've been placed_
    let idx1 = packets.binary_search(&div1).unwrap_err();
    // would've been shifted forward because of div1
    let idx2 = packets.binary_search(&div2).unwrap_err() + 1;

    // AOC wants 1-based indexing
    Ok((idx1 + 1) * (idx2 + 1))
}

#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
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

    #[test_case("[3,4]" => L(vec![I(3),I(4)]))]
    #[test_case("[]" => L(vec![]))]
    #[test_case("[[3],[]]" => L(vec![L(vec![I(3)]), L(vec![])]))]
    #[test_case("[[],[[]],[1]]" => L(vec![L(vec![]), L(vec![L(vec![])]), L(vec![I(1)])]))]
    #[test_case("[[[[]]],[]]" => L(vec![L(vec![L(vec![L(vec![])])]), L(vec![])]))]
    #[test_case("[[1],[2,3,4]]" => L(vec![L(vec![I(1)]),L(vec![I(2),I(3),I(4)])]))]
    #[test_case("[[1],4]" => L(vec![L(vec![I(1)]),I(4)]))]
    fn parse_item(s: &str) -> Item {
        s.parse::<Item>().unwrap()
    }

    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 13)]
    #[test_case(REAL => 5503)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 140)]
    #[test_case(REAL => 20952)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
