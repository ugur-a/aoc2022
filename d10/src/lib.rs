use aoc2022lib::impl_from_str_from_nom_parser;

use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i32, combinator::map,
    sequence::preceded, IResult,
};
use std::{collections::BTreeMap, str::FromStr};

#[derive(Clone, Copy)]
enum Operation {
    Addx(i32),
    Noop,
}

fn noop(input: &str) -> IResult<&str, Operation> {
    map(tag("noop"), |_| Operation::Noop)(input)
}

fn addx(input: &str) -> IResult<&str, Operation> {
    map(preceded(tag("addx "), i32), Operation::Addx)(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((noop, addx))(input)
}

impl_from_str_from_nom_parser!(operation, Operation);

fn operations(file: &str, init_value: i32) -> anyhow::Result<BTreeMap<usize, i32>> {
    let mut register_history = BTreeMap::new();
    let mut cycle = 0;
    let mut register_value = init_value;
    for line in file.lines() {
        match Operation::from_str(line)? {
            Operation::Addx(num) => {
                cycle += 2;
                register_value += num;
                register_history.insert(cycle, register_value);
            }
            Operation::Noop => {
                cycle += 1;
            }
        }
    }
    Ok(register_history)
}

trait BiggestPrevious<Q> {
    type Item;
    fn biggest_previous(&self, query: Q) -> Option<&Self::Item>;
}

impl BiggestPrevious<usize> for BTreeMap<usize, i32> {
    type Item = i32;
    fn biggest_previous(&self, query: usize) -> Option<&Self::Item> {
        self.range(..=query).next_back().map(|(_u, i)| i)
    }
}

pub fn p1(file: &str) -> anyhow::Result<i32> {
    let interesting_cycles = (20..=220).step_by(40);

    let register_history = operations(file, 1)?;

    let res = interesting_cycles
        .map(|cycle| cycle as i32 * register_history.biggest_previous(cycle - 1).unwrap())
        .sum();
    Ok(res)
}

pub fn p2(file: &str) -> anyhow::Result<String> {
    struct Crt {
        width: usize,
        height: usize,
    }

    let crt = Crt {
        width: 40,
        height: 6,
    };

    let register_history = operations(file, 1)?;

    let res = (0..crt.height)
        .map(|row_num| {
            (0..crt.width)
                .map(|col_num| {
                    let cycle = crt.width * row_num + col_num;

                    // only check against the horizontal position of the sprite
                    let crt_position = col_num;

                    let center_of_sprite = register_history.biggest_previous(cycle).unwrap();

                    if center_of_sprite.abs_diff(crt_position as i32) <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n");
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");
    const P2_OUT_EXAMPLE: &str = include_str!("../outputs/p2/example.txt");
    const P2_OUT_REAL: &str = include_str!("../outputs/p2/real.txt");

    #[test_case(EXAMPLE => 13140)]
    #[test_case(REAL => 15360)]
    fn test_p1(inp: &str) -> i32 {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => P2_OUT_EXAMPLE)]
    #[test_case(REAL => P2_OUT_REAL)]
    fn test_p2(inp: &str) -> String {
        p2(inp).unwrap()
    }
}
