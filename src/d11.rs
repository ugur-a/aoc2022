use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};


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

pub fn p1(file: &str) -> Result<u32> {
    todo!()
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
        assert_eq!(p1(&inp).unwrap(), 10605);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d11/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
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
