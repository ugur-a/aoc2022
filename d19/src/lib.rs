use std::str::FromStr;

use anyhow::{Error, Result};

pub fn p1(file: &str) -> Result<u32> {
    todo!()
}
pub fn p2(_file: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 33);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
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
