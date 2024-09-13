use anyhow::{bail, Context};
use aoc2022lib::impl_from_str_for_obj_with_lifetimes_from_nom_parser;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, u64},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

type Number = u64;

type Name<'a> = &'a str;

fn name(i: &str) -> IResult<&str, Name> {
    take(4usize)(i)
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Div,
    Sub,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            s => bail!("Invalid operation: {s}"),
        }
    }
}

fn operation(i: &str) -> IResult<&str, Operation> {
    map_res(take(1usize), str::parse)(i)
}

trait ApplyOperation<T> {
    fn apply_operation(self, operation: Operation, other: T) -> T;
}

impl<T> ApplyOperation<T> for T
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    fn apply_operation(self, operation: Operation, other: T) -> T {
        match operation {
            Operation::Add => self + other,
            Operation::Sub => self - other,
            Operation::Mul => self * other,
            Operation::Div => self / other,
        }
    }
}

enum Job<'a> {
    Number(Number),
    Calculate {
        monkey_1st: Name<'a>,
        operation: Operation,
        monkey_2nd: Name<'a>,
    },
}

fn job(i: &str) -> IResult<&str, Job> {
    alt((
        map(u64, Job::Number),
        map(
            tuple((
                name,
                preceded(char(' '), operation),
                preceded(char(' '), name),
            )),
            |(monkey_1st, operation, monkey_2nd)| Job::Calculate {
                monkey_1st,
                operation,
                monkey_2nd,
            },
        ),
    ))(i)
}

struct Monkey<'a> {
    name: Name<'a>,
    job: Job<'a>,
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    map(separated_pair(name, tag(": "), job), |(name, job)| Monkey {
        name,
        job,
    })(i)
}

impl_from_str_for_obj_with_lifetimes_from_nom_parser!(monkey, Monkey);

struct Monkeys<'a> {
    monkeys: HashMap<Name<'a>, Job<'a>>,
}

impl<'input, 'output> TryFrom<&'input str> for Monkeys<'output>
where
    'input: 'output,
{
    type Error = nom::error::Error<String>;

    fn try_from(s: &'input str) -> Result<Self, Self::Error> {
        let monkeys = s
            .lines()
            .map(Monkey::try_from)
            .map_ok(|monkey| (monkey.name, monkey.job))
            .try_collect()?;

        Ok(Self { monkeys })
    }
}

impl<'a> Monkeys<'a> {
    fn number(&self, name: &str) -> Option<Number> {
        let job = self.monkeys.get(name)?;

        match *job {
            Job::Number(num) => Some(num),
            Job::Calculate {
                monkey_1st,
                operation,
                monkey_2nd,
            } => {
                let num_1st = self.number(monkey_1st)?;
                let num_2nd = self.number(monkey_2nd)?;
                let num = num_1st.apply_operation(operation, num_2nd);
                Some(num)
            }
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<Number> {
    let monkeys = Monkeys::try_from(file)?;
    let number = monkeys.number("root").context("No root in list")?;
    Ok(number)
}
pub fn p2(_file: &str) -> anyhow::Result<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 152)]
    #[test_case(REAL => 31_017_034_894_002)]
    fn test_p1(inp: &str) -> u64 {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => ignore 301)]
    #[test_case(REAL => ignore 0)]
    fn test_p2(inp: &str) -> u64 {
        p2(inp).unwrap()
    }
}
