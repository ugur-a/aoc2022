use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use anyhow::{bail, Context};
use aoc2022lib::impl_from_str_from_nom_parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, u64},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

type Number = u64;

type Name = String;

fn name(i: &str) -> IResult<&str, Name> {
    map(take(4usize), str::to_string)(i)
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

enum Job {
    Number(Number),
    Calculate {
        monkey_1st: Name,
        operation: Operation,
        monkey_2nd: Name,
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

struct Monkey {
    name: Name,
    job: Job,
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    map(separated_pair(name, tag(": "), job), |(name, job)| Monkey {
        name,
        job,
    })(i)
}

impl_from_str_from_nom_parser!(monkey, Monkey);

struct Monkeys {
    monkeys: HashMap<Name, Job>,
}

impl FromStr for Monkeys {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys: Vec<Monkey> = Vec::with_capacity(s.lines().count());

        for line in s.lines() {
            let monkey = Monkey::from_str(line)?;
            monkeys.push(monkey);
        }

        let monkeys = monkeys.into_iter().map(|m| (m.name, m.job)).collect();

        Ok(Self { monkeys })
    }
}

impl Monkeys {
    fn number(&self, name: &str) -> Option<Number> {
        let job = self.monkeys.get(name)?;

        match job {
            &Job::Number(num) => Some(num),
            Job::Calculate {
                monkey_1st,
                operation,
                monkey_2nd,
            } => {
                let num_1st = self.number(monkey_1st)?;
                let num_2nd = self.number(monkey_2nd)?;
                let num = num_1st.apply_operation(*operation, num_2nd);
                Some(num)
            }
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<Number> {
    let monkeys = Monkeys::from_str(file)?;
    let number = monkeys.number("root").context("No root in list")?;
    Ok(number)
}
pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 152);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 31_017_034_894_002);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
