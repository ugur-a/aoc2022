use std::str::FromStr;

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
        let (x, y) = match self.direction {
            Direction::Right => ((x + time).a(), y.a()),
            Direction::Left => ((x - time).a(), y.a()),
            Direction::Up => (x.a(), (y - time).a()),
            Direction::Down => (x.a(), (y + time).a()),
        };
        Point2D(x, y)
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
                .filter(|c| ['>', '<', '^', 'v'].contains(c))
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
    fn start() -> ValleyPos {
        ValleyPos::Inside(Point2D(0, 0))
    }
    fn end(&self) -> ValleyPos {
        ValleyPos::Inside(Point2D(self.width - 1, self.height - 1))
    }
    fn collides(&self, pos: ValleyPos, time: usize) -> bool {
        match pos {
            ValleyPos::Entrance | ValleyPos::Exit => false,
            ValleyPos::Inside(pos) => self
                .blizzards
                .iter()
                .any(|blizzard| pos == blizzard.pos(time)),
        }
    }

    fn next_positions(
        &self,
        pos: ValleyPos,
        time: usize,
    ) -> impl Iterator<Item = (ValleyPos, usize)> + '_ {
        let positions = match pos {
            ValleyPos::Entrance => {
                vec![ValleyPos::Entrance, Valley::start()]
            }
            vp @ ValleyPos::Inside(Point2D(x, y)) => {
                let mut positions = vec![ValleyPos::Inside(Point2D(x, y))];
                if vp == Valley::start() {
                    positions.push(ValleyPos::Entrance);
                }
                if vp == self.end() {
                    positions.push(ValleyPos::Exit);
                }

                if x > 0 {
                    positions.push(ValleyPos::Inside(Point2D(x - 1, y)));
                }
                if x < self.width - 1 {
                    positions.push(ValleyPos::Inside(Point2D(x + 1, y)));
                }
                if y > 0 {
                    positions.push(ValleyPos::Inside(Point2D(x, y - 1)));
                }
                if y < self.height - 1 {
                    positions.push(ValleyPos::Inside(Point2D(x, y + 1)));
                }
                positions
            }
            ValleyPos::Exit => {
                vec![ValleyPos::Exit, self.end()]
            }
        };

        positions
            .into_iter()
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
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 295);
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
