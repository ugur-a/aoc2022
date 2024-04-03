use std::ops::{Add, Mul};
use std::str::FromStr;

use anyhow::{bail, Context};
use itertools::Itertools;
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

#[derive(Clone, Copy)]
enum Operand {
    Add,
    Mul,
}

#[derive(Clone, Copy)]
enum Value<N> {
    Old,
    Number(N),
}

impl<N> FromStr for Monkey<N>
where
    N: FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut note_lines = s.lines().skip(1);
        let starting_items = note_lines
            .next()
            .context("no starting items")?
            .strip_prefix("  Starting items: ")
            .context("invalid input")?
            .split(", ")
            .map(N::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let operation = note_lines
            .next()
            .context("no operations")?
            .strip_prefix("  Operation: new = old ")
            .context("invalid input")?
            .parse()?;

        let divisible_by = note_lines
            .next()
            .context("no test")?
            .strip_prefix("  Test: divisible by ")
            .context("invalid input")?
            .parse()?;

        let monkey_true = note_lines
            .next()
            .context("no first monkey")?
            .strip_prefix("    If true: throw to monkey ")
            .context("invalid input")?
            .parse()?;

        let monkey_false = note_lines
            .next()
            .context("no second monkey")?
            .strip_prefix("    If false: throw to monkey ")
            .context("invalid input")?
            .parse()?;

        Ok(Self {
            inventory: starting_items,
            operation,
            divisible_by,
            monkey_true,
            monkey_false,
            activity: 0,
        })
    }
}

#[derive(Clone, Copy)]
struct Operation<N: Copy> {
    operand: Operand,
    value: Value<N>,
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
                operand: Operand::Add,
                value: Value::Number(value),
            } => self + value,
            Operation {
                operand: Operand::Mul,
                value: Value::Number(value),
            } => self * value,
            Operation {
                operand: Operand::Add,
                value: Value::Old,
            } => self + self,
            Operation {
                operand: Operand::Mul,
                value: Value::Old,
            } => self * self,
        }
    }
}

impl<N> FromStr for Operation<N>
where
    N: FromStr + Copy,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operation = s.split_whitespace();
        let operand = match operation.next().context("no operator")? {
            "+" => Operand::Add,
            "*" => Operand::Mul,
            _ => bail!("invalid operator"),
        };

        let value = operation.next().context("no operand")?;
        let value = {
            if value == "old" {
                Value::Old
            } else if let Ok(num) = value.parse::<N>() {
                Value::Number(num)
            } else {
                bail!("invalid value");
            }
        };

        Ok(Operation { operand, value })
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
