use std::str::FromStr;

use anyhow::{Error, Result};

pub fn p1(file: &str) -> Result<usize> {
    todo!()
}
pub fn p2(_file: &str) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d17/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3068);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d17/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d17/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d17/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
