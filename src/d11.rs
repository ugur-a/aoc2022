use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

struct Monkey {
    inventory: Vec<u32>,
    operation: Operation,
    divisible_by: u32,
    monkey_to_throw_to_if_test_true: u32,
    monkey_to_throw_to_if_test_false: u32,
    activity: u32,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut note_lines = s.lines().skip(1);
        let starting_items = note_lines
            .next()
            .context("no starting items")?
            .strip_prefix("  Starting items: ")
            .context("invalid input")?
            .split(", ")
            .map(|elem| elem.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        let operation = note_lines
            .next()
            .context("no operations")?
            .strip_prefix("  Operation: new = old ")
            .context("invalid input")?
            .parse::<Operation>()?;

        let divisible_by = note_lines
            .next()
            .context("no test")?
            .strip_prefix("  Test: divisible by ")
            .context("invalid input")?
            .parse::<u32>()?;

        let monkey_to_throw_to_if_test_true = note_lines
            .next()
            .context("no first monkey")?
            .strip_prefix("   If true: throw to monkey ")
            .context("invalid input")?
            .parse::<u32>()?;

        let monkey_to_throw_to_if_test_false = note_lines
            .next()
            .context("no second monkey")?
            .strip_prefix("    If false: throw to monkey ")
            .context("invalid input")?
            .parse::<u32>()?;

        Ok(Self {
            inventory: starting_items,
            operation,
            divisible_by,
            monkey_to_throw_to_if_test_true,
            monkey_to_throw_to_if_test_false,
            activity: 0,
        })
    }
}

enum Operation {
    Add(u32),
    Mul(u32),
    Square,
    Double,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operation = s.split_whitespace();
        let operand = operation.next().context("no operator")?;
        let value = operation.next().context("no operand")?;
        use Operation as O;
        if let Ok(num) = value.parse::<u32>() {
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

pub fn p1(file: &str, num_rounds: u32) -> Result<u32> {
    let monkeys = file
        .split("\n\n")
        .map(|monkey_notes| monkey_notes.parse::<Monkey>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(monkeys
        .iter()
        .map(|monkey| monkey.activity)
        .sorted_unstable()
        .rev()
        .take(2)
        .product())
}

pub fn p2(file: &str) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d11/test.txt").unwrap();
        assert_eq!(p1(&inp, 20).unwrap(), 10605);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d11/real.txt").unwrap();
        assert_eq!(p1(&inp, 20).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d11/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), ());
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d11/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), ());
    }
}
