use anyhow::Context;

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
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .enumerate()
        .map(|(idx, n)| Number::new(n, idx))
        .collect();

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

pub fn p2(_file: &str) -> anyhow::Result<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 8764);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 1_623_178_306);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 535_648_840_980);
    }
}
