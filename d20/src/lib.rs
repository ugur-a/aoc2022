use anyhow::Context;
use itertools::Itertools;

struct Number {
    value: i64,
    order: usize,
}

impl Number {
    fn new(value: i64, order: usize) -> Self {
        Self { value, order }
    }
}

trait Mix {
    fn mix(&mut self) -> anyhow::Result<&mut Self>;
}

impl Mix for Vec<Number> {
    fn mix(&mut self) -> anyhow::Result<&mut Self> {
        let len = self.len();
        for order in 0..len {
            let position = self
                .iter()
                .position(|number| number.order == order)
                .expect("I just put you there");
            let number = self.remove(position);
            let new_position = {
                let position: i64 = position.try_into()?;
                let len: i64 = len.try_into()?;
                let new_position = (position + number.value).rem_euclid(len - 1);
                new_position.try_into()?
            };
            self.insert(new_position, number);
        }
        Ok(self)
    }
}

pub fn p1(file: &str) -> anyhow::Result<i64> {
    let mut numbers: Vec<Number> = file
        .lines()
        .enumerate()
        .map(|(idx, n)| n.parse().map(|n| Number::new(n, idx)))
        .try_collect()?;

    numbers.mix()?;

    let idx_of_zero = numbers
        .iter()
        .position(|number| number.value == 0)
        .context("No 0 in list")?;

    let res = [1000, 2000, 3000]
        .into_iter()
        .map(|position| (idx_of_zero + position) % numbers.len())
        .map(|position| &numbers[position])
        .map(|number| number.value)
        .sum();

    Ok(res)
}

pub fn p2(file: &str) -> anyhow::Result<i64> {
    const NUM_MIXES: i32 = 10;
    const DECRYPTION_KEY: i64 = 811_589_153;

    let mut numbers: Vec<Number> = file
        .lines()
        .enumerate()
        .map(|(i, n)| n.parse().map(|n: i64| Number::new(DECRYPTION_KEY * n, i)))
        .try_collect()?;

    for _ in 0..NUM_MIXES {
        numbers.mix()?;
    }

    let idx_of_zero = numbers
        .iter()
        .position(|number| number.value == 0)
        .context("No 0 in list")?;

    let res = [1000, 2000, 3000]
        .into_iter()
        .map(|position| (idx_of_zero + position) % numbers.len())
        .map(|position| &numbers[position])
        .map(|number| number.value)
        .sum();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 3)]
    #[test_case(REAL => 8764)]
    fn test_p1(inp: &str) -> i64 {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 1_623_178_306)]
    #[test_case(REAL => 535_648_840_980)]
    fn test_p2(inp: &str) -> i64 {
        p2(inp).unwrap()
    }
}
