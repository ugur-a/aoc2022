use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    iter::repeat,
    str::FromStr,
};

use anyhow::bail;
use aoc2022lib::{impl_from_str_from_nom_parser, points::Point2D};
use derive_deref::Deref;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Point = Point2D<u32>;

struct Border {
    left: u32,
    right: u32,
    down: u32,
}

#[derive(Clone, Copy)]
enum UnitType {
    Sand,
    Stone,
}

struct Cave {
    borders: Border,
    resting: HashMap<Point, UnitType>,
}

// 498,4
fn point(i: &str) -> IResult<&str, Point> {
    map(separated_pair(u32, char(','), u32), |(x, y)| Point2D(x, y))(i)
}

#[derive(Deref)]
struct Path(Vec<Point>);

// 498,4 -> 498,6 -> 496,6
fn path(i: &str) -> IResult<&str, Path> {
    map(separated_list1(tag(" -> "), point), Path)(i)
}

impl_from_str_from_nom_parser!(path, Path);

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths = s
            .lines()
            .map(Path::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let borders = {
            let (left, right) = paths
                .iter()
                .flat_map(|path| path.iter())
                .map(Point2D::x)
                .minmax()
                .into_option()
                .unwrap();
            let down = paths
                .iter()
                .flat_map(|path| path.iter())
                .map(Point2D::y)
                .max()
                .unwrap();
            Border { left, right, down }
        };

        let mut resting: HashMap<Point, UnitType> = HashMap::new();
        for path in paths {
            // FIXME use array_windows once that's stabilized
            // https://github.com/rust-lang/rust/issues/75027
            for pair in path.windows(2) {
                let &[p1, p2] = pair else { unreachable!() };
                let points = all_points_between_two_points(p1, p2)?;
                resting.extend(points.zip(repeat(UnitType::Stone)));
            }
        }

        Ok(Self { borders, resting })
    }
}

fn all_points_between_two_points(
    p1 @ Point2D(x1, y1): Point,
    p2 @ Point2D(x2, y2): Point,
) -> anyhow::Result<Box<dyn Iterator<Item = Point2D<u32, u32>>>> {
    if y1 == y2 {
        let res = (min(x1, x2)..=max(x1, x2))
            .zip(repeat(y1))
            .map(|(x, y)| Point2D(x, y));
        Ok(Box::new(res))
    } else if x1 == x2 {
        let res = repeat(x1)
            .zip(min(y1, y2)..=max(y1, y2))
            .map(|(x, y)| Point2D(x, y));
        Ok(Box::new(res))
    } else {
        bail!("points are not on a line: {p1:#?}, {p2:#?}");
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for y in 0..=(self.borders.down + 2) {
            for x in (self.borders.left - 2)..=(self.borders.right + 2) {
                let char = match self.resting.get(&Point2D(x, y)) {
                    Some(UnitType::Stone) => '#',
                    Some(UnitType::Sand) => 'o',
                    None => '.',
                };
                res.push(char);
            }
            res.push('\n');
        }
        write!(f, "{res}")
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let mut cave = file.parse::<Cave>()?;

    let init_sand = Point2D(500, 0);
    let mut sands = 0;
    'outer: loop {
        let mut sand = init_sand;

        while let Some(next_sand) = [
            Point2D(sand.0, sand.1 + 1),
            Point2D(sand.0 - 1, sand.1 + 1),
            Point2D(sand.0 + 1, sand.1 + 1),
        ]
        .into_iter()
        .find(|point| !cave.resting.contains_key(point))
        {
            let sand_in_bounds = (cave.borders.left..=cave.borders.right).contains(&sand.0)
                && (..cave.borders.down).contains(&sand.1);

            if !sand_in_bounds {
                break 'outer;
            }
            sand = next_sand;
        }
        cave.resting.insert(sand, UnitType::Sand);
        sands += 1;
    }

    Ok(sands)
}
pub fn p2(file: &str) -> anyhow::Result<u32> {
    let mut cave = file.parse::<Cave>()?;

    let init_sand = Point2D(500, 0);
    let mut sands = 0;
    while !cave.resting.contains_key(&init_sand) {
        let mut sand = init_sand;
        // while:
        // the next point downwards isn't on the Ultimate Lower Border
        while sand.1 + 1 < cave.borders.down + 2 {
            // and there's somewhere to fall to
            // comment: these two conditions (where+if let) should really be
            // checked simulatenously but this is not stable yet
            // (Reference: `https://github.com/rust-lang/rust/issues/53667`)
            if let Some(next_sand) = [
                Point2D(sand.0, sand.1 + 1),
                Point2D(sand.0 - 1, sand.1 + 1),
                Point2D(sand.0 + 1, sand.1 + 1),
            ]
            .into_iter()
            .find(|point| !cave.resting.contains_key(point))
            {
                // fall
                sand = next_sand;
            } else {
                break;
            }
        }
        cave.resting.insert(sand, UnitType::Sand);
        sands += 1;
    }

    Ok(sands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 24);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 897);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 93);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 26683);
    }
}
