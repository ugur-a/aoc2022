use std::str::FromStr;

use anyhow::{Error, Result};

pub fn p1(file: &str, row_to_analyze: usize) -> Result<usize> {
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
        let inp = read_to_string("inputs/d14/test.txt").unwrap();
        assert_eq!(p1(&inp, 10).unwrap(), 26);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p1(&inp, 2000000).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d14/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
