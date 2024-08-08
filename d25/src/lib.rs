use std::{fmt::Display, str::FromStr};

use anyhow::bail;

enum SnafuDigit {
    MinusTwo,
    MinusOne,
    Zero,
    One,
    Two,
}

impl TryFrom<char> for SnafuDigit {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(Self::MinusTwo),
            '-' => Ok(Self::MinusOne),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            c => bail!("unknown char: {c}"),
        }
    }
}

impl From<&SnafuDigit> for char {
    fn from(value: &SnafuDigit) -> Self {
        match *value {
            SnafuDigit::MinusTwo => '=',
            SnafuDigit::MinusOne => '-',
            SnafuDigit::Zero => '0',
            SnafuDigit::One => '1',
            SnafuDigit::Two => '2',
        }
    }
}

impl From<SnafuDigit> for i64 {
    fn from(value: SnafuDigit) -> Self {
        match value {
            SnafuDigit::MinusTwo => -2,
            SnafuDigit::MinusOne => -1,
            SnafuDigit::Zero => 0,
            SnafuDigit::One => 1,
            SnafuDigit::Two => 2,
        }
    }
}

struct Snafu {
    digits: Vec<SnafuDigit>,
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars().rev() {
            let d = SnafuDigit::try_from(c)?;
            digits.push(d);
        }
        Ok(Self { digits })
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.digits.iter().rev().map(char::from).collect();
        write!(f, "{s}")
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        value
            .digits
            .into_iter()
            .rev()
            .map(i64::from)
            .fold(0, |num, d| 5 * num + d)
    }
}

impl From<i64> for Snafu {
    fn from(mut value: i64) -> Self {
        let mut digits = vec![];
        while value != 0 {
            value += 2;
            let d = value % 5;
            let s_d = match d {
                0 => SnafuDigit::MinusTwo,
                1 => SnafuDigit::MinusOne,
                2 => SnafuDigit::Zero,
                3 => SnafuDigit::One,
                4 => SnafuDigit::Two,
                _ => unreachable!(),
            };
            digits.push(s_d);
            value /= 5;
        }
        Self { digits }
    }
}

pub fn p1(file: &str) -> anyhow::Result<String> {
    let mut sum = 0;
    for line in file.lines() {
        let snafu = Snafu::from_str(line)?;
        let num = i64::from(snafu);
        sum += num;
    }
    let snafu = Snafu::from(sum);
    Ok(snafu.to_string())
}
pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use test_case::test_case;

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(3, "1=")]
    #[test_case(4, "1-")]
    #[test_case(5, "10")]
    #[test_case(6, "11")]
    #[test_case(7, "12")]
    #[test_case(8, "2=")]
    #[test_case(9, "2-")]
    #[test_case(10, "20")]
    #[test_case(15, "1=0")]
    #[test_case(20, "1-0")]
    #[test_case(2022, "1=11-2")]
    #[test_case(12345, "1-0---0")]
    #[test_case(314_159_265, "1121-1110-1=0")]
    fn dec2snafu(inp: i64, out: &str) {
        let num = Snafu::from(inp);
        assert_eq!(num.to_string(), out);
    }

    #[test_case("1=-0-2", 1747)]
    #[test_case("12111", 906)]
    #[test_case("2=0=", 198)]
    #[test_case("21", 11)]
    #[test_case("2=01", 201)]
    #[test_case("111", 31)]
    #[test_case("20012", 1257)]
    #[test_case("112", 32)]
    #[test_case("1=-1=", 353)]
    #[test_case("1-12", 107)]
    #[test_case("12", 7)]
    #[test_case("1=", 3)]
    #[test_case("122", 37)]
    fn snafu2dec(inp: &str, out: i64) {
        let snafu = Snafu::from_str(inp).unwrap();
        assert_eq!(i64::from(snafu), out);
    }

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), "2=-1=0");
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), "2-=2==00-0==2=022=10");
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
