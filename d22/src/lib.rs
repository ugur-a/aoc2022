use bare_metal_modulo::{MNum, ModNum};
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use aoc2022lib::{impl_from_str_from_nom_parser, points::Point2D};
use derive_deref::Deref;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map, multi::many1,
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Point {
    Air,
    Tile,
    Wall,
}

impl TryFrom<char> for Point {
    type Error = anyhow::Error;
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            ' ' => Ok(Point::Air),
            '.' => Ok(Point::Tile),
            '#' => Ok(Point::Wall),
            c => bail!(c),
        }
    }
}

pub struct Map {
    height: usize,
    width: usize,
    inner: Vec<Vec<Point>>,
}

impl Map {
    pub fn first_free_position(&self) -> Point2D<ModNum<usize>> {
        self.inner[0]
            .iter()
            .position(|p| matches!(p, Point::Tile))
            .map(|col| Point2D(ModNum::new(0, self.height), ModNum::new(col, self.width)))
            .expect("first row must contain open tiles")
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().map(str::len).max().context("input is empty")?;

        let map: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Point::try_from).try_collect())
            .try_collect()?;

        Ok(Self {
            height,
            width,
            inner: map,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum FacingDirection {
    Right,
    Down,
    Left,
    Up,
}

pub struct You<'a> {
    map: &'a Map,
    position: Point2D<ModNum<usize>>,
    direction: FacingDirection,
}

#[derive(Clone, Copy, Debug)]
pub enum TurnDirection {
    Left,
    Right,
}

impl FromStr for TurnDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(TurnDirection::Right),
            "L" => Ok(TurnDirection::Left),
            c => bail!("invalid turn direction: {c}"),
        }
    }
}

impl<'a> You<'a> {
    pub fn new(map: &'a Map) -> Self {
        let position = map.first_free_position();
        Self {
            map,
            position,
            direction: FacingDirection::Right,
        }
    }

    pub fn turn(&mut self, turn_direction: TurnDirection) {
        use FacingDirection as D;
        use TurnDirection as TD;
        self.direction = match (&self.direction, turn_direction) {
            (D::Up, TD::Right) | (D::Down, TD::Left) => D::Right,
            (D::Right, TD::Right) | (D::Left, TD::Left) => D::Down,
            (D::Down, TD::Right) | (D::Up, TD::Left) => D::Left,
            (D::Left, TD::Right) | (D::Right, TD::Left) => D::Up,
        };
    }

    fn go(&mut self, distance: usize) {
        // let r#move = move |mut row: ModNum<usize>, mut col: ModNum<usize>| {
        //     let direction = self.direction.clone();
        //     match direction {
        //         FacingDirection::Right => col += 1,
        //         FacingDirection::Down => row += 1,
        //         FacingDirection::Left => col -= 1,
        //         FacingDirection::Up => row -= 1,
        //     };
        // };
        let mut went = 0;
        let Point2D(row, col) = &mut self.position;
        let (mut next_row, mut next_col) = (*row, *col);
        while went < distance {
            match self.direction {
                FacingDirection::Right => next_col += 1,
                FacingDirection::Down => next_row += 1,
                FacingDirection::Left => next_col -= 1,
                FacingDirection::Up => next_row -= 1,
            };

            // nothing / empty space in the map
            while let Some(Point::Air) | None = &self.map.inner[next_row.a()].get(next_col.a()) {
                match self.direction {
                    FacingDirection::Right => next_col += 1,
                    FacingDirection::Down => next_row += 1,
                    FacingDirection::Left => next_col -= 1,
                    FacingDirection::Up => next_row -= 1,
                };
            }

            #[allow(clippy::match_on_vec_items)]
            match &self.map.inner[next_row.a()].get(next_col.a()) {
                None | Some(Point::Air) => unreachable!(),
                Some(Point::Wall) => break,
                Some(Point::Tile) => {
                    *row = next_row;
                    *col = next_col;
                }
            };
            went += 1;
        }
    }

    pub fn into_password(self) -> usize {
        let Point2D(row, col) = self.position;
        let direction = match self.direction {
            FacingDirection::Right => 0,
            FacingDirection::Down => 1,
            FacingDirection::Left => 2,
            FacingDirection::Up => 3,
        };
        1000 * (row.a() + 1) + 4 * (col.a() + 1) + direction
    }
}

#[derive(Deref)]
pub struct LabyrinthPath(Vec<Action>);

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Go(usize),
    Turn(TurnDirection),
}

fn action(i: &str) -> IResult<&str, Action> {
    alt((
        map(tag("R"), |_| Action::Turn(TurnDirection::Right)),
        map(tag("L"), |_| Action::Turn(TurnDirection::Left)),
        map(u32, |distance| Action::Go(distance as usize)),
    ))(i)
}

pub fn labyrinth_path(i: &str) -> IResult<&str, LabyrinthPath> {
    map(many1(action), LabyrinthPath)(i)
}

impl_from_str_from_nom_parser!(labyrinth_path, LabyrinthPath);

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let (map, path) = file
        .split_once("\n\n")
        .with_context(|| format!("couldn't split into map and path: {file}"))?;
    let map = Map::from_str(map)?;
    let mut you = You::new(&map);
    let path = LabyrinthPath::from_str(path)?;
    for action in &*path {
        match &action {
            Action::Turn(direction) => you.turn(*direction),
            Action::Go(distance) => {
                you.go(*distance);
            }
        }
    }
    Ok(you.into_password())
}

pub fn p2(_file: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    use super::*;
    use FacingDirection::{Left as L, Right as R, Up as U};
    use Point::{Air as A, Tile as T, Wall as W};

    #[test_case(0, 1, 4, &[&[T,T,T,W]]
    )]
    // ...#
    //    ##
    #[test_case(1,2,5,&[&[T,T,T,W],&[A,A,A,W,W]])]
    // ...#
    //    ##
    //    ...#
    #[test_case(2, 3, 7,&[
        &[T,T,T,W],
        &[A,A,A,W,W],
        &[A,A,A,T,T,T,W],
    ])]
    #[test_case(9,5,16,&[
        &[A,A,A,A,A,A,A,A,T,T,T,W],
        &[A,A,A,A,A,A,A,A,T,T,T,T],
        &[T,T,T,W,T,T,T,T,T,T,T,W],
        &[A,A,A,A,A,A,A,A,T,T,T,W,T,T,T,T],
        &[A,A,A,A,A,A,A,A,T,T,T,T,T,T,W,T],
    ])]
    fn parse_map(num: usize, height: usize, width: usize, contents: &[&[Point]]) {
        let inp = read_to_string(format!("tests/maps/{num}")).unwrap();
        let map = Map::from_str(&inp).unwrap();
        assert_eq!(map.height, height);
        assert_eq!(map.width, width);
        assert_eq!(map.inner, contents);
    }

    #[test]
    fn spawn() {
        let map = Map {
            height: 5,
            width: 16,
            inner: vec![
                vec![A, A, A, A, A, A, A, A, T, T, T, W],
                vec![A, A, A, A, A, A, A, A, T, T, T, T],
                vec![T, T, T, W, T, T, T, T, T, T, T, W],
                vec![A, A, A, A, A, A, A, A, T, T, T, W, T, T, T, T],
                vec![A, A, A, A, A, A, A, A, T, T, T, T, T, T, W, T],
            ],
        };
        let you = You::new(&map);
        let Point2D(row, col) = you.position;
        assert_eq!(row, ModNum::new(0, 5));
        assert_eq!(col, ModNum::new(8, 16));
    }

    #[test_case(0,1,R,1,0,2;"1 right")]
    #[test_case(0,1,R,2,0,2;"2 right;hit wall")]
    #[test_case(0,1,L,1,0,1;"1 left; hit wall")]
    #[test_case(0,1,U,1,0,1;"1 up")]
    fn go_easy(
        row: usize,
        col: usize,
        direction: FacingDirection,
        distance: usize,
        row_after: usize,
        col_after: usize,
    ) {
        let mut you = You {
            position: Point2D(ModNum::new(row, 1), ModNum::new(col, 4)),
            direction,
            map: &Map {
                height: 1,
                width: 4,
                inner: vec![vec![W, T, T, W]],
            },
        };
        you.go(distance);
        assert_eq!(you.position.0.a(), row_after);
        assert_eq!(you.position.1.a(), col_after);
    }

    #[test_case(0,8,R,0,0,8;"don't move")]
    #[test_case(0,8,R,1,0,9;"dist 1")]
    #[test_case(0,8,R,2,0,10;"dist 2")]
    #[test_case(0,8,R,3,0,10;"dist 3; hit a wall")]
    #[test_case(0,8,R,4,0,10;"dist 4; hit a wall")]
    #[test_case(0,8,L,1,0,8;"left, dist 1; hit a wall after air (goes outside first row's borders")]
    fn go(
        row: usize,
        col: usize,
        direction: FacingDirection,
        distance: usize,
        row_after: usize,
        col_after: usize,
    ) {
        let mut you = You {
            position: Point2D(ModNum::new(row, 5), ModNum::new(col, 16)),
            direction,
            map: &Map {
                height: 5,
                width: 16,
                inner: vec![
                    vec![A, A, A, A, A, A, A, A, T, T, T, W],
                    vec![A, A, A, A, A, A, A, A, T, T, T, T],
                    vec![T, T, T, W, T, T, T, T, T, T, T, W],
                    vec![A, A, A, A, A, A, A, A, T, T, T, W, T, T, T, T],
                    vec![A, A, A, A, A, A, A, A, T, T, T, T, T, T, W, T],
                ],
            },
        };
        you.go(distance);
        assert_eq!(you.position.0.a(), row_after);
        assert_eq!(you.position.1.a(), col_after);
    }

    #[test_case(EXAMPLE => 6032)]
    #[test_case(REAL => 27492)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => ignore 5031)]
    #[test_case(REAL => ignore 0)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
