use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use itertools::{repeat_n, Itertools};
use num::Integer;

struct Monkey<N> {
    inventory: Vec<N>,
    operation: Operation<N>,
    divisible_by: N,
    monkey_true: usize,
    monkey_false: usize,
    activity: usize,
}

impl<N> FromStr for Monkey<N>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut note_lines = s.lines().skip(1);
        let starting_items = note_lines
            .next()
            .context("no starting items")?
            .strip_prefix("  Starting items: ")
            .context("invalid input")?
            .split(", ")
            .map(|elem| elem.parse())
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

enum Operation<N> {
    Add(N),
    Mul(N),
    Square,
    Double,
}

impl<N> FromStr for Operation<N>
where
    N: FromStr,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operation = s.split_whitespace();
        let operand = operation.next().context("no operator")?;
        let value = operation.next().context("no operand")?;
        use Operation as O;
        if let Ok(num) = value.parse() {
            match operand {
                "+" => Ok(O::Add(num)),
                "*" => Ok(O::Mul(num)),
                _ => bail!("invalid operator"),
            }
        } else {
            match value {
                "old" => match operand {
                    "+" => Ok(O::Double),
                    "*" => Ok(O::Square),
                    _ => bail!("invalid operator"),
                },
                _ => bail!("invalid operand"),
            }
        }
    }
}

pub fn p1(file: &str, num_rounds: u32) -> Result<usize> {
    let mut monkeys = file
        .split("\n\n")
        .map(|monkey_notes| monkey_notes.parse::<Monkey<u32>>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut inventories_to_transfer = repeat_n(Vec::new(), monkeys.len()).collect_vec();
    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.extend(
                inventories_to_transfer
                    .get_mut(idx)
                    .context("there should've been an empty vector here if nothing else")?
                    .drain(..),
            );

            monkey.activity += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u32>, Vec<u32>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| match monkey.operation {
                    Operation::Add(num) => item_worry + num,
                    Operation::Mul(num) => item_worry * num,
                    Operation::Square => item_worry.pow(2),
                    Operation::Double => item_worry * 2,
                })
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

pub fn p2(file: &str, num_rounds: u32) -> Result<usize> {
    let mut monkeys = file
        .split("\n\n")
        .map(|monkey_notes| monkey_notes.parse::<Monkey<u64>>())
        .collect::<Result<Vec<_>, _>>()?;

    let divisibility_tests_lcm = monkeys
        .iter()
        .map(|monkey| monkey.divisible_by)
        .reduce(|acc, e| acc.lcm(&e))
        .context("non-zero amount of monkeys")?;

    let mut inventories_to_transfer = repeat_n(Vec::new(), monkeys.len()).collect_vec();
    for _ in 0..num_rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.inventory.extend(
                inventories_to_transfer
                    .get_mut(idx)
                    .context("there should've been an empty vector here if nothing else")?
                    .drain(..),
            );

            monkey.activity += monkey.inventory.len();

            let (items_monkey_true, items_monkey_false): (Vec<u64>, Vec<u64>) = monkey
                .inventory
                .drain(..)
                // monkey applies its operation
                .map(|item_worry| match monkey.operation {
                    Operation::Add(num) => item_worry + num,
                    Operation::Mul(num) => item_worry * num,
                    Operation::Square => item_worry.pow(2),
                    Operation::Double => item_worry * 2,
                })
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
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d11/test.txt").unwrap();
        assert_eq!(p1(&inp, 20).unwrap(), 10_605);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d11/real.txt").unwrap();
        assert_eq!(p1(&inp, 20).unwrap(), 54_054);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/d11/test.txt").unwrap();
        assert_eq!(p2(&inp, 10000).unwrap(), 2_713_310_158);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/d11/real.txt").unwrap();
        assert_eq!(p2(&inp, 10000).unwrap(), 0);
    }
}
