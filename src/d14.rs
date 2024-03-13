use std::{
    cmp::{max, min},
    collections::HashSet,
    iter::repeat,
    str::FromStr,
};

use anyhow::{Error, Result};
use itertools::Itertools;

struct Border {
    left: u32,
    right: u32,
    down: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point2D(u32, u32);

struct Cave {
    borders: Border,
    resting: HashSet<Point2D>,
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (left, right) = s
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|point| point.split_once(',').unwrap().0)
            })
            .minmax()
            .into_option()
            .unwrap();
        let left = left.parse::<u32>()?;
        let right = right.parse::<u32>()?;

        let down = s
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|point| point.split_once(',').unwrap().1)
            })
            .max()
            .unwrap()
            .parse::<u32>()?;
        let borders = Border { left, right, down };

        let mut resting: HashSet<Point2D> = HashSet::new();
        for line in s.lines() {
            let line = line
                .split(" -> ")
                .map(|point| point.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
                .map(|(x, y)| Point2D(x, y))
                .collect::<Vec<_>>();

            for pair in line.windows(2) {
                let &[Point2D(x1, y1), Point2D(x2, y2)] = pair else {
                    unreachable!()
                };
                let points: Vec<Point2D> = if y1 == y2 {
                    (min(x1, x2)..=max(x1, x2))
                        .zip(repeat(y1))
                        .map(|(x, y)| Point2D(x, y))
                        .collect()
                } else {
                    repeat(x1)
                        .zip(min(y1, y2)..=max(y1, y2))
                        .map(|(x, y)| Point2D(x, y))
                        .collect()
                };

                resting.extend(points);
            }
        }

        Ok(Self { borders, resting })
    }
}

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
        let inp = read_to_string("inputs/d14/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
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
