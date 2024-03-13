use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
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

enum UnitType {
    Sand,
    Stone,
}

struct Cave {
    borders: Border,
    resting: HashMap<Point2D, UnitType>,
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
                    .map(|point| point.split_once(',').unwrap().1.parse::<u32>().unwrap())
            })
            .max()
            .unwrap();
        let borders = Border { left, right, down };

        let mut resting: HashMap<Point2D, UnitType> = HashMap::new();
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

                resting.extend(points.iter().map(|&point| (point, UnitType::Stone)));
            }
        }

        Ok(Self { borders, resting })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for y in 0..=self.borders.down {
            for x in self.borders.left..=self.borders.right {
                let char = match self.resting.get(&Point2D(x, y)) {
                    Some(UnitType::Stone) => '#',
                    Some(UnitType::Sand) => 'o',
                    None => '.',
                };
                res.push(char);
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

pub fn p1(file: &str) -> Result<u32> {
    let mut cave = file.parse::<Cave>()?;
    // println!("{}", cave);

    let init_sand = Point2D(500, 0);
    let mut sands = 0;
    'outer: loop {
        let mut sand = init_sand;
        'inner: loop {
            if sand.1 > cave.borders.down
                || sand.0 < cave.borders.left
                || cave.borders.right < sand.0
            // || (sand.1..=cave.borders.down)
            //     .map(|y| Point2D(sand.0, y))
            //     .all(|point| !cave.resting.contains(&point))
            {
                break 'outer;
            }
            if !cave.resting.get(&Point2D(sand.0, sand.1 + 1)).is_some() {
                sand.1 += 1;
            } else if !cave.resting.get(&Point2D(sand.0 - 1, sand.1 + 1)).is_some() {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !cave.resting.get(&Point2D(sand.0 + 1, sand.1 + 1)).is_some() {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                cave.resting.insert(sand, UnitType::Sand);
                sands += 1;
                break 'inner;
            }
        }
    }
    // println!("{}", cave);
    // println!("{}", sands);
    Ok(sands)
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
        assert_eq!(p1(&inp).unwrap(), 24);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 897);
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
