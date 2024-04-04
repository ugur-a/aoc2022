use std::ops::{Add, Mul};
use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
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
    activity: usize,
}

fn parse_n<N: FromStr>(input: &str) -> IResult<&str, N> {
    map_res(digit1, N::from_str)(input)
}

// 79, 98
fn parse_starting_items<N: FromStr>(input: &str) -> IResult<&str, Vec<N>> {
    separated_list0(tag(", "), parse_n)(input)
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
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

fn parse_operand<N: FromStr + Copy>(input: &str) -> IResult<&str, Operand<N>> {
    alt((
        value(Operand::Old, tag("old")),
        map(parse_n, Operand::Number),
    ))(input)
}

#[derive(Clone, Copy)]
struct Operation<N: Copy> {
    operator: Operator,
    operand: Operand<N>,
}

// new = old * 19
fn parse_operation<N: FromStr + Copy>(input: &str) -> IResult<&str, Operation<N>> {
    map(
        preceded(
            tag("new = old "),
            separated_pair(parse_operator, char(' '), parse_operand),
        ),
        |(operator, operand)| Operation { operator, operand },
    )(input)
}

fn parse_divisible_by<N: FromStr + Copy>(input: &str) -> IResult<&str, N> {
    preceded(tag("divisible by "), parse_n)(input)
}

// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn parse_monkey<N: FromStr + Copy>(input: &str) -> IResult<&str, Monkey<N>> {
    map(
        preceded(
            tuple((tag("Monkey "), digit1, tag(":"), newline)),
            tuple((
                delimited(
                    tag("  Starting items: "),
                    parse_starting_items::<N>,
                    newline,
                ),
                delimited(tag("  Operation: "), parse_operation::<N>, newline),
                delimited(tag("  Test: "), parse_divisible_by::<N>, newline),
                delimited(tag("    If true: "), parse_n, newline),
                delimited(tag("    If false: "), parse_n, newline),
            )),
        ),
        |(starting_items, operation, divisible_by, monkey_true, monkey_false)| Monkey::<N> {
            inventory: starting_items,
            operation,
            divisible_by,
            monkey_true,
            monkey_false,
            activity: 0,
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
        match parse_monkey(s).finish() {
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
            Operation {
                operator: Operator::Add,
                operand: Operand::Number(value),
            } => self + value,
            Operation {
                operator: Operator::Mul,
                operand: Operand::Number(value),
            } => self * value,
            Operation {
                operator: Operator::Add,
                operand: Operand::Old,
            } => self + self,
            Operation {
                operator: Operator::Mul,
                operand: Operand::Old,
            } => self * self,
        }
    }
}

pub fn p1(file: &str, num_rounds: u32) -> anyhow::Result<usize> {
    let mut monkeys = file
        .split("\n\n")
        .map(Monkey::<u32>::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut inventories_to_transfer = vec![Vec::new(); monkeys.len()];
    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.append(
                inventories_to_transfer
                    .get_mut(idx)
                    .context("there should've been an empty vector here if nothing else")?,
            );

            if monkey.inventory.is_empty() {
                continue;
            }

            monkey.activity += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u32>, Vec<u32>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| item_worry.apply_operation(monkey.operation))
                // your worry level decreases
                .map(|item_worry| item_worry / 3)
                // monkey inspects each item
                .partition(|item_worry| item_worry % monkey.divisible_by == 0);

            inventories_to_transfer
                .get_mut(monkey.monkey_true)
                .with_context(|| format!("no monkey number {}", monkey.monkey_true))?
                .extend(items_monkey_true);
            inventories_to_transfer
                .get_mut(monkey.monkey_false)
                .with_context(|| format!("no monkey number {}", monkey.monkey_true))?
                .extend(items_monkey_false);
        }
    }

    Ok(monkeys
        .iter()
        .map(|monkey| monkey.activity)
        .sorted_unstable()
        .rev()
        .take(2)
        .product())
}

pub fn p2(file: &str, num_rounds: u32) -> anyhow::Result<usize> {
    let mut monkeys = file
        .split("\n\n")
        .map(Monkey::<u64>::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let divisibility_tests_lcm = monkeys
        .iter()
        .map(|monkey| monkey.divisible_by)
        .reduce(|acc, e| acc.lcm(&e))
        .context("non-zero amount of monkeys")?;

    let mut inventories_to_transfer = vec![Vec::new(); monkeys.len()];
    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.append(
                inventories_to_transfer
                    .get_mut(idx)
                    .context("there should've been an empty vector here if nothing else")?,
            );

            if monkey.inventory.is_empty() {
                continue;
            }

            monkey.activity += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u64>, Vec<u64>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| item_worry.apply_operation(monkey.operation))
                .map(|item_worry| item_worry % divisibility_tests_lcm)
                // monkey inspects each item
                .partition(|item_worry| item_worry % monkey.divisible_by == 0);

            inventories_to_transfer
                .get_mut(monkey.monkey_true)
                .with_context(|| format!("no monkey number {}", monkey.monkey_true))?
                .extend(items_monkey_true);
            inventories_to_transfer
                .get_mut(monkey.monkey_false)
                .with_context(|| format!("no monkey number {}", monkey.monkey_true))?
                .extend(items_monkey_false);
        }
    }

    Ok(monkeys
        .iter()
        .map(|monkey| monkey.activity)
        .sorted_unstable()
        .rev()
        .take(2)
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/d11/test.txt");
        assert_eq!(p1(inp, 20).unwrap(), 10_605);
    }
    #[test]
    fn real_p1() {
        let inp = include_str!("../inputs/d11/real.txt");
        assert_eq!(p1(inp, 20).unwrap(), 54_054);
    }
    #[test]
    fn test_p2() {
        let inp = include_str!("../inputs/d11/test.txt");
        assert_eq!(p2(inp, 10000).unwrap(), 2_713_310_158);
    }
    #[test]
    fn real_p2() {
        let inp = include_str!("../inputs/d11/real.txt");
        assert_eq!(p2(inp, 10000).unwrap(), 14_314_925_001);
    }
}
