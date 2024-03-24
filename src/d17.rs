use std::{collections::HashSet, fmt::Display, thread::sleep, time::Duration};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

use crate::points::Point2D;
#[derive(Clone, Copy)]
struct Rock {
    points: [Point2D<u32>; 5],
    width: u32,
}

#[derive(Clone, Copy)]
enum RockType {
    Minus,
    Plus,
    RightL,
    I,
    Square,
}

macro_rules! rock {
    [$( ( $p1:expr, $p2:expr ) ),+] => {[$( Point2D($p1, $p2) ),+]};
}

impl From<RockType> for Rock {
    fn from(r#type: RockType) -> Self {
        let (points, width) = match r#type {
            RockType::Minus => (rock![(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)], 4),
            RockType::Plus => (rock![(1, 0), (0, 1), (2, 1), (1, 2), (1, 1)], 3),
            RockType::RightL => (rock![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3),
            RockType::I => (rock![(0, 0), (0, 1), (0, 2), (0, 3), (0, 0)], 1),
            RockType::Square => (rock![(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)], 2),
        };
        Self { points, width }
    }
}

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
            chr => Err(anyhow!("Invalid char: '{}'", chr)),
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

    fn add_points(&mut self, points: &[Point2D<u32>; 5]) {
        self.occupied_points.extend(points);
    }

    fn highest_point(&self) -> u32 {
        self.occupied_points
            .iter()
            .map(|Point2D(_x, y)| *y)
            .max()
            .map_or(0, |y| y + 1)
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = (0..=self.highest_point())
            .rev()
            .map(|y| {
                (0..=self.width)
                    .map(move |x| (Point2D(x, y)))
                    .map(|point| {
                        if self.occupied_points.contains(&point) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n");

        write!(f, "{res}")
    }
}

pub fn p1(file: &str) -> Result<u32> {
    let num_rounds = 2022;
    let mut chamber = Chamber::new(7);
    use RockType as RT;
    let rocks = vec![RT::Minus, RT::Plus, RT::RightL, RT::I, RT::Square]
        .into_iter()
        .map(Rock::from)
        .cycle()
        .take(num_rounds);
    let mut pushes = file
        .chars()
        .map(|s| JetStreamDirection::try_from(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .cycle();

    for rock in rocks {
        let spawn_height = chamber
            .occupied_points
            .iter()
            .map(|Point2D(_x, y)| *y)
            .max()
            .map_or(3, |height| height + 1 + 3);
        let mut rock_position_relative = Point2D(2, spawn_height);
        loop {
            println!("{}\n", chamber);
            // jet stream
            match pushes.next().unwrap() {
                JetStreamDirection::Left => {
                    if rock_position_relative.0 > 0
                        && rock
                            .points
                            .iter()
                            .map(|point| *point + rock_position_relative)
                            .map(|Point2D(x, y)| Point2D(x - 1, y))
                            .all(|point| !chamber.contains(&point))
                    {
                        rock_position_relative.0 -= 1;
                    }
                }
                JetStreamDirection::Right => {
                    if rock_position_relative.0 + rock.width < chamber.width
                        && rock
                            .points
                            .iter()
                            .map(|point| *point + rock_position_relative)
                            .map(|Point2D(x, y)| Point2D(x + 1, y))
                            .all(|point| !chamber.contains(&point))
                    {
                        rock_position_relative.0 += 1;
                    }
                }
            }
            // come to rest if:
            // 1) arrived at the lowest point
            if rock_position_relative.1 == 0 {
                chamber.add_points(&rock.points.map(|point| point + rock_position_relative));
                break;
            }
            // 2) there's a rock point directly underneath
            let mut rock_stops = false;
            for &Point2D(x, y) in &rock.points {
                if chamber.contains(
                    &(Point2D(
                        x + rock_position_relative.0,
                        y + rock_position_relative.1 - 1,
                    )),
                ) {
                    rock_stops = true;
                    break;
                }
            }
            if rock_stops {
                chamber.add_points(&rock.points.map(|point| point + rock_position_relative));
                break;
            //fall
            } else {
                rock_position_relative.1 -= 1;
            }
        }
    }
    Ok(chamber.highest_point())
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
    fn real_p1() {
        let inp = read_to_string("inputs/d17/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3206);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/d17/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 1_514_285_714_288);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d17/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
