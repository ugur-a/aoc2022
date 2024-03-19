use std::str::FromStr;

use anyhow::{Error, Result};

use crate::points::Point2D;

macro_rules! rock {
    [$( ( $p1:expr, $p2:expr ) ),+] => {[$( Point2D($p1, $p2) ),+]};
}

const MinusRock: [Point2D<u32>; 4] = rock![(0, 0), (1, 0), (2, 0), (3, 0)];
const PlusRock: [Point2D<u32>; 5] = rock![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
const RightLRock: [Point2D<u32>; 5] = rock![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const IRock: [Point2D<u32>; 4] = rock![(0, 0), (0, 1), (0, 2), (0, 3)];
const SquareRock: [Point2D<u32>; 4] = rock![(0, 0), (0, 1), (1, 0), (1, 1)];

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
