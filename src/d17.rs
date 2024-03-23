use std::{collections::HashSet, fmt::Display};

use anyhow::{Error, Result};
use itertools::Itertools;

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
    let rocks = vec![MINUS_ROCK, PLUS_ROCK, RIGHT_LROCK, IROCK, SQUARE_ROCK]
        .into_iter()
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
        let mut rock_position_in_chamber = Point2D(2, spawn_height);
        loop {
            // jet stream
            match pushes.next().unwrap() {
                JetStreamDirection::Left => {
                    if rock_position_in_chamber.0 > 0 {
                        rock_position_in_chamber.0 -= 1;
                    }
                }
                JetStreamDirection::Right => {
                    if rock_position_in_chamber.0 + rock.width < chamber.width {
                        rock_position_in_chamber.0 += 1;
                    }
                }
            }
            // come to rest if:
            // 1) arrived at the lowest point
            if rock_position_in_chamber.1 == 0 {
                chamber.add_rock(rock.moved_by_relative_offset(rock_position_in_chamber));
                break;
            }
            // 2) there's a rock point directly underneath
            let mut rock_stops = false;
            for &Point2D(x, y) in &rock.rock_points {
                if chamber.contains(
                    &(Point2D(
                        x + rock_position_in_chamber.0,
                        y + rock_position_in_chamber.1 - 1,
                    )),
                ) {
                    rock_stops = true;
                    break;
                }
            }
            if rock_stops {
                chamber.add_rock(rock.moved_by_relative_offset(rock_position_in_chamber));
                break;
            //fall
            } else {
                rock_position_in_chamber.1 -= 1;
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
