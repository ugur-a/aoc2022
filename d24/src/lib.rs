use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Context};
use aoc2022lib::points::{ManhattanDistance, Point2D};
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
    blizzards_right: HashSet<Pos>,
    blizzards_left: HashSet<Pos>,
    blizzards_up: HashSet<Pos>,
    blizzards_down: HashSet<Pos>,
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

        let mut blizzards_right = HashSet::new();
        let mut blizzards_left = HashSet::new();
        let mut blizzards_up = HashSet::new();
        let mut blizzards_down = HashSet::new();

        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, char) in line.chars().skip(1).enumerate() {
                let b = match char {
                    '.' | '#' => continue,
                    _ => Point2D(x, y),
                };
                let bs = match Direction::try_from(char)? {
                    Direction::Right => &mut blizzards_right,
                    Direction::Left => &mut blizzards_left,
                    Direction::Up => &mut blizzards_up,
                    Direction::Down => &mut blizzards_down,
                };
                bs.insert(b);
            }
        }

        Ok(Self {
            width,
            height,
            blizzards_right,
            blizzards_left,
            blizzards_up,
            blizzards_down,
        })
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
                let b_right = Point2D((mod_x - time).a(), y);
                let b_left = Point2D((mod_x + time).a(), y);
                let b_up = Point2D(x, (mod_y + time).a());
                let b_down = Point2D(x, (mod_y - time).a());

                self.blizzards_right.contains(&b_right)
                    || self.blizzards_left.contains(&b_left)
                    || self.blizzards_up.contains(&b_up)
                    || self.blizzards_down.contains(&b_down)
            }
        }
    }

    fn next_positions(
        &self,
        pos: ValleyPos,
        time: usize,
    ) -> impl Iterator<Item = (ValleyPos, usize)> + '_ {
        let next_positions = match pos {
            ValleyPos::Entrance => vec![Valley::start()],
            ValleyPos::Exit => vec![self.end()],
            vp @ ValleyPos::Inside(Point2D(x, y)) => std::iter::empty()
                .chain((vp == Valley::start()).then_some(ValleyPos::Entrance))
                .chain((vp == self.end()).then_some(ValleyPos::Exit))
                .chain((x > 0).then(|| ValleyPos::Inside(Point2D(x - 1, y))))
                .chain((y > 0).then(|| ValleyPos::Inside(Point2D(x, y - 1))))
                .chain((x < self.width - 1).then_some(ValleyPos::Inside(Point2D(x + 1, y))))
                .chain((y < self.height - 1).then_some(ValleyPos::Inside(Point2D(x, y + 1))))
                .collect(),
        };

        std::iter::once(pos)
            .chain(next_positions)
            .map(move |pos| (pos, time + 1))
            .filter(|&(pos, time)| !self.collides(pos, time))
    }

    fn manhattan_distance(&self, p1: ValleyPos, p2: ValleyPos) -> usize {
        match (p1, p2) {
            // trivial
            (ValleyPos::Inside(s), ValleyPos::Inside(o)) => s.manhattan_distance(o),
            (ValleyPos::Exit, ValleyPos::Exit) | (ValleyPos::Entrance, ValleyPos::Entrance) => 0,
            // d(Entrance, X) = d(Entrance, Start) + d(Start, X) = 1 + d(Start, X)
            (ValleyPos::Entrance, o) => 1 + self.manhattan_distance(Valley::start(), o),
            // d(X, Exit) = d(X, End) + d(End, Exit) = d(X, End) + 1
            (s, ValleyPos::Exit) => self.manhattan_distance(s, self.end()) + 1,
            // maintain cmp order Entrance->Inside->Exit
            (s, o @ ValleyPos::Entrance) | (s @ ValleyPos::Exit, o) => {
                self.manhattan_distance(o, s)
            }
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let valley = Valley::from_str(file)?;
    let mut time = 0;
    let start = ValleyPos::Entrance;
    let destination = ValleyPos::Exit;
    let (_, time1) = astar::astar(
        &(start, time),
        |&(pos, time)| valley.next_positions(pos, time).map(|pt| (pt, 1)),
        |&(pos, _)| valley.manhattan_distance(pos, destination),
        |&(pos, _)| pos == destination,
    )
    .context("no path found")?;
    time = time1;
    Ok(time)
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let valley = Valley::from_str(file)?;

    let mut time = 0;

    // there
    let mut start = ValleyPos::Entrance;
    let mut destination = ValleyPos::Exit;
    let (_, time1) = astar::astar(
        &(start, time),
        |&(pos, time)| valley.next_positions(pos, time).map(|pt| (pt, 1)),
        |&(pos, _)| valley.manhattan_distance(pos, destination),
        |&(pos, _)| pos == destination,
    )
    .context("no path found")?;
    time += time1;

    // back
    std::mem::swap(&mut start, &mut destination);

    let (_, time2) = astar::astar(
        &(start, time),
        |&(pos, time)| valley.next_positions(pos, time).map(|pt| (pt, 1)),
        |&(pos, _)| valley.manhattan_distance(pos, destination),
        |&(pos, _)| pos == destination,
    )
    .context("no path found")?;
    time += time2;

    // there again
    std::mem::swap(&mut start, &mut destination);

    let (_, time3) = astar::astar(
        &(start, time),
        |&(pos, time)| valley.next_positions(pos, time).map(|pt| (pt, 1)),
        |&(pos, _)| valley.manhattan_distance(pos, destination),
        |&(pos, _)| pos == destination,
    )
    .context("no path found")?;
    time += time3;

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
