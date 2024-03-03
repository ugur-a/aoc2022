use std::{collections::HashSet, iter, ops::Neg, str::FromStr};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default, Debug)]
struct Point2D {
    x: i32,
    y: i32,
}
impl Point2D {
    fn new() -> Self {
        Self::default()
    }

    fn r#move(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

pub fn p1(file: &str) -> usize {
    let mut rope = Rope::with_length(2);
    file.lines()
        .flat_map(|r#move| {
            let (direction, num_repeats) = r#move.split_once(' ').unwrap();
            let direction = direction.parse::<Direction>().unwrap();
            let num_repeats = num_repeats.parse::<usize>().unwrap();
            iter::repeat(direction).take(num_repeats)
        })
        .map(|direction| {
            rope.r#move(direction);
            *rope.last().unwrap()
        })
        .chain(iter::once(Point2D::new()))
        .collect::<HashSet<_>>()
        .len()
}

type Rope = Vec<Point2D>;
trait RopeTrait {
    fn with_length(len: usize) -> Self;
    fn r#move(&mut self, direction: Direction);
}

impl RopeTrait for Rope {
    fn with_length(len: usize) -> Self {
        iter::repeat(Point2D::new()).take(len).collect::<Vec<_>>()
    }
    fn r#move(&mut self, direction: Direction) {
        let mut new_rope: Rope = Vec::with_capacity(self.len());
        let mut early_break = false;
        for curr in self.iter_mut() {
            match new_rope.last() {
                // no previous knot, current knot is head
                None => curr.r#move(direction),
                // there's a previous knot, i.e. the current knot is not the head
                Some(prev) => {
                    // check the distance to the previous knot
                    // - if too big, teleport to it
                    if curr.x.abs_diff(prev.x) > 1 || curr.y.abs_diff(prev.y) > 1 {
                        *curr = *prev;
                        curr.r#move(-direction);
                    // - otherwise:
                    // 1. don't move at all
                    // 2. observe that all the necessary pulls have already been made
                    // and the rest of the rope doesn't need to move, so don't bother checking it
                    // TODO: implement a check for that
                    } else {
                        early_break = true;
                        break;
                    }
                }
            }
            new_rope.push(*curr);
        }
        if early_break {
            // update the moved knots' coordinates, leave the rest alone
            self.splice(..new_rope.len(), new_rope);
        } else {
            *self = new_rope;
        }
    }
}

pub fn p2(file: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn move_2_rope() {
        let mut rope = Rope::with_length(2);
        rope.r#move(Direction::Up);
        assert_eq!(rope, vec![Point2D { x: 0, y: 1 }, Point2D { x: 0, y: 0 }]);
    }
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d9/test.txt").unwrap();
        assert_eq!(p1(&inp), 13);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d9/real.txt").unwrap();
        assert_eq!(p1(&inp), 5960);
    }
}
