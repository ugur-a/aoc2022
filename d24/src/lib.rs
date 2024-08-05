use std::str::FromStr;

use anyhow::{bail, Context};
use aoc2022lib::points::{ManhattanDistance, Point2D};
use pathfinding::directed::astar;

type Pos = Point2D<usize>;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            c => bail!("invalid direction: {c}"),
        }
    }
}

struct Blizzard {
    pos: Pos,
    direction: Direction,
}

type Collider = Box<dyn Fn(Pos, usize) -> bool>;

impl Blizzard {
    #[allow(clippy::needless_pass_by_value)]
    fn into_collider(self, dimension: usize) -> Collider {
        let Point2D(x, y) = self.pos;
        match self.direction {
            Direction::Right => {
                Box::new(move |pos, time| pos == Point2D((x + time) % dimension, y))
            }
            Direction::Left => Box::new(move |pos, time| pos == Point2D((x - time) % dimension, y)),
            Direction::Up => Box::new(move |pos, time| pos == Point2D(x, (y - time) % dimension)),
            Direction::Down => Box::new(move |pos, time| pos == Point2D(x, (y - time) % dimension)),
        }
    }
}

struct Valley {
    width: usize,
    height: usize,
    colliders: Vec<Collider>,
}

impl FromStr for Valley {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .context("input has no lines (is empty)")?
            .len()
            - 2;
        let height = s.lines().count() - 2;

        let mut colliders = Vec::with_capacity(
            s.chars()
                .filter(|c| matches!(*c, '>' | '<' | '^' | 'v'))
                .count(),
        );

        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, char) in line.chars().skip(1).enumerate() {
                match char {
                    '.' | '#' => continue,
                    c => {
                        let blizzard = Blizzard {
                            pos: Point2D(x, y),
                            direction: Direction::try_from(c)?,
                        };

                        let collider = match c {
                            '>' | '<' => blizzard.into_collider(width),
                            '^' | 'v' => blizzard.into_collider(height),
                            _ => unreachable!("checked while creating `blizzard`"),
                        };
                        colliders.push(collider);
                    }
                }
            }
        }

        Ok(Self {
            width,
            height,
            colliders,
        })
    }
}

impl Valley {
    fn collides(&self, pos: Pos, time: usize) -> bool {
        self.colliders.iter().any(|collider| collider(pos, time))
    }

    fn next_positions(&self, pos: Pos, time: usize) -> impl Iterator<Item = (Pos, usize)> + '_ {
        let Point2D(x, y) = pos;
        let mut positions = vec![(x, y)];
        if x > 0 {
            positions.push((x - 1, y));
        }
        if x < self.width - 1 {
            positions.push((x + 1, y));
        }
        if y > 0 {
            positions.push((x, y - 1));
        }
        if y < self.height - 1 {
            positions.push((x, y + 1));
        }

        positions
            .into_iter()
            .map(move |(x, y)| (Point2D(x, y), time + 1))
            .filter(|&(pos, time)| !self.collides(pos, time))
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let valley = Valley::from_str(file)?;
    let start = (Point2D(0, 0), 1);
    let destination = Point2D(valley.width - 1, valley.height - 1);
    let (_, len) = astar::astar(
        &start,
        |&(pos, time)| valley.next_positions(pos, time).map(|pt| (pt, 1)),
        |&(pos, _)| pos.manhattan_distance(destination),
        |&(pos, _)| pos == destination,
    )
    .context("no path found")?;
    Ok(len + 2)
}

pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 18);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
