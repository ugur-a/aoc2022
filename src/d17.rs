use std::{collections::HashSet, fmt::Display};

use anyhow::{Error, Result};

use crate::points::Point2D;
#[derive(Clone, Copy)]
struct Rock {
    rock_points: [Point2D<u32>; 5],
    width: u32,
}

impl Rock {
    fn moved_by_relative_offset(&self, relative_offset: Point2D<u32>) -> Self {
        let new_rock_points: [Point2D<u32>; 5] = self
            .rock_points
            .map(|relative_position_in_rock| relative_position_in_rock + relative_offset);
        Self {
            rock_points: new_rock_points,
            width: self.width,
        }
    }
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

#[derive(Clone, Copy)]
enum JetStreamDirection {
    Left,
    Right,
}

impl TryFrom<char> for JetStreamDirection {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => unimplemented!(),
        }
    }
}

struct Chamber {
    width: u32,
    occupied_points: HashSet<Point2D<u32>>,
}

impl Chamber {
    fn new(width: u32) -> Self {
        Self {
            width,
            occupied_points: HashSet::new(),
        }
    }

    fn contains(&self, q: &Point2D<u32>) -> bool {
        self.occupied_points.contains(q)
    }

    fn add_rock(&mut self, rock: Rock) {
        self.occupied_points.extend(rock.rock_points);
    }

    fn highest_point(&self) -> u32 {
        self.occupied_points
            .iter()
            .map(|Point2D(_x, y)| *y)
            .max()
            .map_or(0, |y| y + 1)
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
