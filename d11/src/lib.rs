use std::ops::{Add, Mul};
use std::str::FromStr;

use anyhow::Context;
use aoc2022lib::parse::n;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    Finish, IResult,
};
use num::Integer;

#[allow(clippy::struct_field_names)]
struct Monkey<N: Copy> {
    inventory: Vec<N>,
    operation: Operation<N>,
    divisible_by: N,
    monkey_true: usize,
    monkey_false: usize,
}

// 79, 98
fn starting_items<N: FromStr>(input: &str) -> IResult<&str, Vec<N>> {
    separated_list0(tag(", "), n)(input)
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Mul, char('*')),
        value(Operator::Add, char('+')),
    ))(input)
}

#[derive(Clone, Copy)]
enum Operand<N> {
    Old,
    Number(N),
}

fn operand<N: FromStr + Copy>(input: &str) -> IResult<&str, Operand<N>> {
    alt((value(Operand::Old, tag("old")), map(n, Operand::Number)))(input)
}

#[derive(Clone, Copy)]
struct Operation<N: Copy>(Operator, Operand<N>);

// new = old * 19
fn operation<N: FromStr + Copy>(input: &str) -> IResult<&str, Operation<N>> {
    map(
        preceded(
            tag("new = old "),
            separated_pair(operator, char(' '), operand),
        ),
        |(operator, operand)| Operation(operator, operand),
    )(input)
}

fn divisible_by<N: FromStr + Copy>(input: &str) -> IResult<&str, N> {
    preceded(tag("divisible by "), n)(input)
}

fn usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

fn throw_to(input: &str) -> IResult<&str, usize> {
    preceded(tag("throw to monkey "), usize)(input)
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn monkey<N: FromStr + Copy>(input: &str) -> IResult<&str, Monkey<N>> {
    map(
        tuple((
            delimited(tag("Monkey "), digit1, tag(":")),
            preceded(tag("\n  Starting items: "), starting_items::<N>),
            preceded(tag("\n  Operation: "), operation::<N>),
            preceded(tag("\n  Test: "), divisible_by::<N>),
            preceded(tag("\n    If true: "), throw_to),
            preceded(tag("\n    If false: "), throw_to),
        )),
        |(_, starting_items, operation, divisible_by, monkey_true, monkey_false)| Monkey::<N> {
            inventory: starting_items,
            operation,
            divisible_by,
            monkey_true,
            monkey_false,
        },
    )(input)
}

impl<N> FromStr for Monkey<N>
where
    N: FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match monkey(s).finish() {
            Ok((_remaining, monkey)) => Ok(monkey),
            Err(nom::error::Error { input, code }) => Err(Self::Err {
                input: input.to_string(),
                code,
            }),
        }
    }
}

trait ApplyOperation {
    fn apply_operation(self, operation: Operation<Self>) -> Self
    where
        Self: Sized + Copy;
}

impl<N> ApplyOperation for N
where
    N: Add<Output = Self> + Mul<Output = Self> + Copy,
{
    fn apply_operation(self, operation: Operation<Self>) -> Self {
        match operation {
            Operation(Operator::Add, Operand::Number(value)) => self + value,
            Operation(Operator::Mul, Operand::Number(value)) => self * value,
            Operation(Operator::Add, Operand::Old) => self + self,
            Operation(Operator::Mul, Operand::Old) => self * self,
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    const NUM_ROUNDS: usize = 20;
    let mut monkeys: Vec<_> = file
        .split("\n\n")
        .map(Monkey::<u32>::from_str)
        .try_collect()?;

    let mut activities: Vec<usize> = vec![0; monkeys.len()];
    let mut inventories_to_transfer = vec![vec![]; monkeys.len()];
    for _ in 0..NUM_ROUNDS {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.append(&mut inventories_to_transfer[idx]);

            if monkey.inventory.is_empty() {
                continue;
            }

            activities[idx] += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u32>, Vec<u32>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| item_worry.apply_operation(monkey.operation))
                // your worry level decreases
                .map(|item_worry| item_worry / 3)
                // monkey inspects each item
                .partition(|item_worry| item_worry.is_multiple_of(&monkey.divisible_by));

            inventories_to_transfer[monkey.monkey_true].extend(items_monkey_true);
            inventories_to_transfer[monkey.monkey_false].extend(items_monkey_false);
        }
    }

    Ok(activities
        .into_iter()
        .sorted_unstable()
        .rev()
        .take(2)
        .product())
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    const NUM_ROUNDS: usize = 10000;
    let mut monkeys: Vec<_> = file
        .split("\n\n")
        .map(Monkey::<u64>::from_str)
        .try_collect()?;

    let divisibility_tests_lcm = monkeys
        .iter()
        .map(|monkey| monkey.divisible_by)
        .reduce(|acc, e| acc.lcm(&e))
        .context("non-zero amount of monkeys")?;

    let mut activities: Vec<usize> = vec![0; monkeys.len()];
    let mut inventories_to_transfer = vec![vec![]; monkeys.len()];
    for _ in 0..NUM_ROUNDS {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.append(&mut inventories_to_transfer[idx]);

            if monkey.inventory.is_empty() {
                continue;
            }

            activities[idx] += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u64>, Vec<u64>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| item_worry.apply_operation(monkey.operation))
                .map(|item_worry| item_worry % divisibility_tests_lcm)
                // monkey inspects each item
                .partition(|item_worry| item_worry.is_multiple_of(&monkey.divisible_by));

            inventories_to_transfer[monkey.monkey_true].extend(items_monkey_true);
            inventories_to_transfer[monkey.monkey_false].extend(items_monkey_false);
        }
    }

    Ok(activities
        .into_iter()
        .sorted_unstable()
        .rev()
        .take(2)
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 10_605)]
    #[test_case(REAL => 54_054)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 2_713_310_158)]
    #[test_case(REAL => 14_314_925_001)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
