use std::{collections::HashSet, iter, str::FromStr};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
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
        use Direction as D;
        match direction {
            D::Up => self.y += 1,
            D::Down => self.y -= 1,
            D::Left => self.x -= 1,
            D::Right => self.x += 1,
            D::UpRight => {
                self.r#move(D::Up);
                self.r#move(D::Right);
            }
            D::UpLeft => {
                self.r#move(D::Up);
                self.r#move(D::Left);
            }
            D::DownRight => {
                self.r#move(D::Down);
                self.r#move(D::Right);
            }
            D::DownLeft => {
                self.r#move(D::Down);
                self.r#move(D::Left);
            }
        };
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
        // take the head and just move it
        let head = self.first_mut().unwrap();
        head.r#move(direction);

        // store the copy of the previous knot to measure the distance to it.
        // would store the reference, but ownership rules
        let mut prev = *head;

        for curr in self.iter_mut().skip(1) {
            use Direction as D;
            let move_to_catch_up = match (prev.x - curr.x, prev.y - curr.y) {
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
            }
            prev = *curr;
        }
    }
}

pub fn p2(file: &str) -> usize {
    let mut rope = Rope::with_length(10);
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
    #[test]
    fn test1_p2() {
        let inp = read_to_string("inputs/d9/test.txt").unwrap();
        assert_eq!(p2(&inp), 1);
    }
    #[test]
    fn test2_p2() {
        let inp = read_to_string("inputs/d9/test2.txt").unwrap();
        assert_eq!(p2(&inp), 36);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/d9/real.txt").unwrap();
        assert_eq!(p2(&inp), 2327);
    }
}
