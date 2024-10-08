use std::{collections::HashSet, iter, str::FromStr};

use anyhow::{bail, Context};
use libaoc::points::Point2D;

type Point = Point2D<i32>;

#[derive(Copy, Clone)]
enum Direction2D {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl FromStr for Direction2D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction2D::Up),
            "D" => Ok(Direction2D::Down),
            "L" => Ok(Direction2D::Left),
            "R" => Ok(Direction2D::Right),
            s => bail!("invalid direction: {s}"),
        }
    }
}

trait Move2D {
    fn r#move(&mut self, direction: Direction2D) -> &mut Self;
}

impl Move2D for Point {
    fn r#move(&mut self, direction: Direction2D) -> &mut Self {
        use Direction2D as D;
        match direction {
            D::Up => self.1 += 1,
            D::Down => self.1 -= 1,
            D::Left => self.0 -= 1,
            D::Right => self.0 += 1,
            D::UpRight => {
                self.r#move(D::Up).r#move(D::Right);
            }
            D::UpLeft => {
                self.r#move(D::Up).r#move(D::Left);
            }
            D::DownRight => {
                self.r#move(D::Down).r#move(D::Right);
            }
            D::DownLeft => {
                self.r#move(D::Down).r#move(D::Left);
            }
        };
        self
    }
}

fn inner(file: &str, rope_len: usize) -> anyhow::Result<usize> {
    let mut rope = Rope::with_length(rope_len);
    let mut movement_directions = Vec::with_capacity(file.lines().count());
    for line in file.lines() {
        let (direction, num_repeats) = line.split_once(' ').context("expected space")?;
        let direction = Direction2D::from_str(direction)?;
        let num_repeats = usize::from_str(num_repeats)?;
        movement_directions.extend(iter::repeat(direction).take(num_repeats));
    }

    let mut visited_positions = HashSet::new();
    visited_positions.insert(Point2D(0, 0));
    for direction in movement_directions {
        rope.r#move(direction);
        visited_positions.insert(*rope.last().unwrap());
    }

    Ok(visited_positions.len())
}

type Rope = Vec<Point>;
trait RopeTrait {
    fn with_length(len: usize) -> Self;
}

impl RopeTrait for Rope {
    fn with_length(len: usize) -> Self {
        vec![Point2D(0, 0); len]
    }
}

impl Move2D for Rope {
    fn r#move(&mut self, direction: Direction2D) -> &mut Self {
        // take the head and just move it
        let head = self.first_mut().unwrap();
        head.r#move(direction);

        // store the copy of the previous knot to measure the distance to it.
        // would store the reference, but ownership rules
        let mut prev = *head;

        for curr in self.iter_mut().skip(1) {
            use Direction2D as D;
            let move_to_catch_up = match (prev.0 - curr.0, prev.1 - curr.1) {
                // knots touch - no catching-up necessary
                (-1..=1, -1..=1) => None,
                // catch-up diagonally
                (1 | 2, 1 | 2) => Some(D::UpRight),
                (-1 | -2, 1 | 2) => Some(D::UpLeft),
                (1 | 2, -1 | -2) => Some(D::DownRight),
                (-1 | -2, -1 | -2) => Some(D::DownLeft),
                // catch-up vertically/horizontally
                (0, 2) => Some(D::Up),
                (0, -2) => Some(D::Down),
                (2, 0) => Some(D::Right),
                (-2, 0) => Some(D::Left),
                _ => unreachable!(),
            };

            match move_to_catch_up {
                // check the distance to the previous knot
                // - if too big, teleport to it
                Some(direction) => curr.r#move(direction),
                // - otherwise:
                // 1. don't move at all
                // 2. observe that all the necessary pulls have already been made and
                //    the rest of the rope doesn't need to move, so don't check further
                None => break,
            };
            prev = *curr;
        }
        self
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    inner(file, 2)
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    inner(file, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EX1: &str = include_str!("../inputs/examples/1");
    const EX2: &str = include_str!("../inputs/examples/2");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test]
    fn move_2_rope() {
        let mut rope = Rope::with_length(2);
        rope.r#move(Direction2D::Up);
        assert_eq!(rope, vec![Point2D(0, 1), Point2D(0, 0)]);
    }
    #[test_case(EX1 => 13)]
    #[test_case(REAL => 5960)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EX1 => 1)]
    #[test_case(EX2 => 36)]
    #[test_case(REAL => 2327)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
