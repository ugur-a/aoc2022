use std::str::FromStr;

use anyhow::{Error, Result};

use crate::points::Point2D;
#[derive(Clone, Copy)]
struct Rock {
    rock_points: [Point2D<u32>; 5],
    width: u32,
}


macro_rules! rock {
    [$( ( $p1:expr, $p2:expr ) ),+] => {[$( Point2D($p1, $p2) ),+]};
}

const MINUS_ROCK: Rock = Rock {
    rock_points: rock![(0, 0), (0, 0), (1, 0), (2, 0), (3, 0)],
    width: 4,
};
const PLUS_ROCK: Rock = Rock {
    rock_points: rock![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    width: 3,
};
const RIGHT_LROCK: Rock = Rock {
    rock_points: rock![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    width: 3,
};
const IROCK: Rock = Rock {
    rock_points: rock![(0, 0), (0, 0), (0, 1), (0, 2), (0, 3)],
    width: 1,
};
const SQUARE_ROCK: Rock = Rock {
    rock_points: rock![(0, 0), (0, 0), (0, 1), (1, 0), (1, 1)],
    width: 2,
};

enum PushDirection {
    Left,
    Right,
}

impl FromStr for PushDirection {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Left),
            ">" => Ok(Self::Right),
            _ => unimplemented!(),
        }
    }
}

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
