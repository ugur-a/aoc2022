use std::{cmp::max, collections::HashSet, fmt::Display};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

use crate::points::Point2D;
#[derive(Clone, Copy)]
struct Rock {
    points: [Point2D<u8, u64>; 5],
    width: u8,
    height: u64,
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

impl Rock {
    fn new(r#type: RockType) -> Self {
        let (points, width, height) = match r#type {
            RockType::Minus => (rock![(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)], 4, 1),
            RockType::Plus => (rock![(1, 0), (0, 1), (2, 1), (1, 2), (1, 1)], 3, 3),
            RockType::RightL => (rock![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3, 3),
            RockType::I => (rock![(0, 0), (0, 1), (0, 2), (0, 3), (0, 0)], 1, 4),
            RockType::Square => (rock![(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)], 2, 2),
        };
        Self {
            points,
            width,
            height,
        }
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

#[derive(Default)]
struct Chamber {
    width: u8,
    height: u64,
    occupied_points: HashSet<Point2D<u8, u64>>,
}

impl Chamber {
    fn new(width: u8) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    fn contains(&self, q: &Point2D<u8, u64>) -> bool {
        self.occupied_points.contains(q)
    }

    fn add_rock(&mut self, rock: Rock, rock_position_relative: Point2D<u8, u64>) {
        self.occupied_points
            .extend(&rock.points.map(|point| point + rock_position_relative));
        self.height = max(self.height, rock_position_relative.1 + rock.height);
    }

    fn height(&self) -> u64 {
        self.height
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = (0..=self.height())
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

fn tetris(file: &str, num_rounds: usize) -> Result<u64> {
    let mut chamber = Chamber::new(7);
    use RockType as RT;
    let rocks = vec![RT::Minus, RT::Plus, RT::RightL, RT::I, RT::Square]
        .into_iter()
        .map(Rock::new)
        .cycle()
        .take(num_rounds);
    let mut pushes = file
        .chars()
        .map(|s| JetStreamDirection::try_from(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .cycle();

    for rock in rocks {
        let spawn_height = chamber.height() + 3;
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
                chamber.add_rock(rock, rock_position_relative);
                break;
            }
            // 2) there's a rock point directly underneath
            let rock_stops = rock
                .points
                .map(|Point2D(x, y)| {
                    Point2D(
                        x + rock_position_relative.0,
                        y + rock_position_relative.1 - 1,
                    )
                })
                .iter()
                .any(|point| chamber.contains(&point));
            if rock_stops {
                chamber.add_rock(rock, rock_position_relative);
                break;
            //fall
            } else {
                rock_position_relative.1 -= 1;
            }
        }
    }
    Ok(chamber.height())
}

pub fn p_mid(file: &str) -> Result<u64> {
    tetris(file, 1_000_000)
}

pub fn p1(file: &str) -> Result<u64> {
    tetris(file, 2022)
}

pub fn p2(file: &str) -> Result<u64> {
    tetris(file, 1_000_000_000_000)
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

    #[test]
    fn test_p_mid() {
        let inp = read_to_string("inputs/d17/real.txt").unwrap();
        assert_eq!(p_mid(&inp).unwrap(), 1_602_842);
    }
}
