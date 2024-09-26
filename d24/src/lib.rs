use std::str::FromStr;

use anyhow::{bail, Context};
use libaoc::points::{ManhattanDistance, Point2D};
use bare_metal_modulo::{MNum, ModNum};
use pathfinding::directed::astar;

type Pos = Point2D<usize>;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ValleyPos {
    Entrance,
    Inside(Pos),
    Exit,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

struct Valley {
    width: usize,
    height: usize,
    map: Vec<Vec<Option<Direction>>>,
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

        let mut map = Vec::with_capacity(height);
        for line in s.lines().skip(1) {
            let mut row_vec = Vec::with_capacity(width);
            for char in line.chars().skip(1) {
                let b = match char {
                    '.' | '#' => None,
                    c => Some(Direction::try_from(c)?),
                };
                row_vec.push(b);
            }
            map.push(row_vec);
        }

        Ok(Self { width, height, map })
    }
}

impl Valley {
    fn start() -> ValleyPos {
        ValleyPos::Inside(Point2D(0, 0))
    }
    fn end(&self) -> ValleyPos {
        ValleyPos::Inside(Point2D(self.width - 1, self.height - 1))
    }
    fn collides(&self, pos: ValleyPos, time: usize) -> bool {
        match pos {
            ValleyPos::Entrance | ValleyPos::Exit => false,
            ValleyPos::Inside(Point2D(x, y)) => {
                let mod_x = ModNum::new(x, self.width);
                let mod_y = ModNum::new(y, self.height);

                // for each direction, calculate where the corresponding blizzard
                // would have needed to start in order to end up @ position @ time
                //
                // e.g. for a rightward blizzard:
                // x(t) == x <=> x(0) == (x - time) % width
                matches!(self.map[y][(mod_x - time).a()], Some(Direction::Right))
                    || matches!(self.map[y][(mod_x + time).a()], Some(Direction::Left))
                    || matches!(self.map[(mod_y + time).a()][x], Some(Direction::Up))
                    || matches!(self.map[(mod_y - time).a()][x], Some(Direction::Down))
            }
        }
    }

    fn next_positions(
        &self,
        pos: ValleyPos,
        time: usize,
    ) -> impl Iterator<Item = (ValleyPos, usize)> + '_ {
        use ValleyPos as VP;
        let next_positions = match pos {
            VP::Entrance => vec![Valley::start()],
            VP::Exit => vec![self.end()],
            vp @ VP::Inside(Point2D(x, y)) => std::iter::empty()
                .chain((vp == Valley::start()).then_some(VP::Entrance))
                .chain((vp == self.end()).then_some(VP::Exit))
                .chain((x > 0).then(|| VP::Inside(Point2D(x - 1, y))))
                .chain((y > 0).then(|| VP::Inside(Point2D(x, y - 1))))
                .chain((x < self.width - 1).then_some(VP::Inside(Point2D(x + 1, y))))
                .chain((y < self.height - 1).then_some(VP::Inside(Point2D(x, y + 1))))
                .collect(),
        };

        std::iter::once(pos)
            .chain(next_positions)
            .map(move |pos| (pos, time + 1))
            .filter(|&(pos, time)| !self.collides(pos, time))
    }

    fn manhattan_distance(&self, p1: ValleyPos, p2: ValleyPos) -> usize {
        use ValleyPos as VP;
        match (p1, p2) {
            // trivial
            (VP::Inside(s), VP::Inside(o)) => s.manhattan_distance(o),
            (VP::Exit, VP::Exit) | (VP::Entrance, VP::Entrance) => 0,

            // d(Entrance, X) = d(Entrance, Start) + d(Start, X) = 1 + d(Start, X)
            (VP::Entrance, o) => 1 + self.manhattan_distance(Valley::start(), o),
            // d(X, Exit) = d(X, End) + d(End, Exit) = d(X, End) + 1
            (s, VP::Exit) => self.manhattan_distance(s, self.end()) + 1,

            // maintain cmp order Entrance->Inside->Exit
            (s, o @ VP::Entrance) | (s @ VP::Exit, o) => self.manhattan_distance(o, s),
        }
    }

    /// Returns the final time
    fn find_path(
        &self,
        start_pos: ValleyPos,
        start_time: usize,
        destination: ValleyPos,
    ) -> anyhow::Result<usize> {
        let (_, time) = astar::astar(
            &(start_pos, start_time),
            |&(pos, time)| self.next_positions(pos, time).map(|pt| (pt, 1)),
            |&(pos, _)| self.manhattan_distance(pos, destination),
            |&(pos, _)| pos == destination,
        )
        .context("no path found")?;
        Ok(start_time + time)
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let valley = Valley::from_str(file)?;

    let time = 0;
    let start = ValleyPos::Entrance;
    let destination = ValleyPos::Exit;

    valley.find_path(start, time, destination)
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let valley = Valley::from_str(file)?;

    let mut time = 0;
    let mut start = ValleyPos::Entrance;
    let mut destination = ValleyPos::Exit;

    // there
    time = valley.find_path(start, time, destination)?;

    // back
    std::mem::swap(&mut start, &mut destination);
    time = valley.find_path(start, time, destination)?;

    // there again
    std::mem::swap(&mut start, &mut destination);
    time = valley.find_path(start, time, destination)?;

    Ok(time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 18)]
    #[test_case(REAL => 295)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 54)]
    #[test_case(REAL => 851)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
