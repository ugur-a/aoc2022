use std::str::FromStr;

use anyhow::{bail, Context};
use aoc2022lib::points::{ManhattanDistance, Point2D};
use bare_metal_modulo::{MNum, ModNum};
use pathfinding::directed::astar;

type Pos = Point2D<usize>;
type PosMod = Point2D<ModNum<usize>>;

#[derive(Debug)]
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
    pos_init: PosMod,
    direction: Direction,
}

impl Blizzard {
    fn pos(&self, time: usize) -> Pos {
        let Point2D(x, y) = self.pos_init;
        match self.direction {
            Direction::Right => Point2D((x + time).a(), y.a()),
            Direction::Left => Point2D((x - time).a(), y.a()),
            Direction::Up => Point2D(x.a(), (y - time).a()),
            Direction::Down => Point2D(x.a(), (y + time).a()),
        }
    }
}

struct Valley {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
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

        let mut blizzards = Vec::with_capacity(
            s.chars()
                .filter(|c| matches!(*c, '>' | '<' | '^' | 'v'))
                .count(),
        );

        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, char) in line.chars().skip(1).enumerate() {
                let b = match char {
                    '.' | '#' => continue,
                    c => Blizzard {
                        pos_init: Point2D(ModNum::new(x, width), ModNum::new(y, height)),
                        direction: Direction::try_from(c)?,
                    },
                };
                blizzards.push(b);
            }
        }

        Ok(Self {
            width,
            height,
            blizzards,
        })
    }
}

impl Valley {
    fn collides(&self, pos: Pos, time: usize) -> bool {
        self.blizzards
            .iter()
            .any(|blizzard| pos == blizzard.pos(time))
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
