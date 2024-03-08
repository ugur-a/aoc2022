use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};
use pathfinding::directed::astar;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point2D {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point2D {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

struct HeightMap {
    start: Point2D,
    goal: Point2D,
    num_rows: usize,
    num_cols: usize,
    heights: HashMap<Point2D, u32>,
}

impl FromStr for HeightMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_row, s_col, _s_char) = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, line)| {
                line.char_indices()
                    .map(move |(col_num, char)| (row_num, col_num, char))
            })
            .find(|(_row_num, _col_num, char)| *char == 'S')
            .context("no starting point found")?;
        let start = Point2D { x: s_col, y: s_row };

        let (g_row, g_col, _g_char) = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, line)| {
                line.char_indices()
                    .map(move |(col_num, char)| (row_num, col_num, char))
            })
            .find(|(_row_num, _col_num, char)| *char == 'E')
            .context("no end point found")?;
        let goal = Point2D { x: g_col, y: g_row };

        let num_cols = s.lines().next().context("at least one row")?.len();

        let num_rows = s.lines().count();

        let heights = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, row)| {
                row.chars()
                    .map(|point| match point {
                        'S' => 'a',
                        'E' => 'z',
                        i @ 'a'..='z' => i,
                        _ => unreachable!(),
                    })
                    .map(|point| point as u32 - 97)
                    .enumerate()
                    .map(move |(col_num, height)| {
                        (
                            Point2D {
                                x: col_num,
                                y: row_num,
                            },
                            height,
                        )
                    })
            })
            .collect::<HashMap<_, _>>();
        Ok(Self {
            start,
            goal,
            num_cols,
            num_rows,
            heights,
        })
    }
}

pub fn p1(file: &str) -> Result<u32> {
    let height_map = file.parse::<HeightMap>()?;
    let shortest_path = astar::astar(
        &height_map.start,
        |&Point2D { x, y }| {
            let this_height = height_map.heights[&Point2D { x, y }];

            let mut potential_neighbours = Vec::new();
            if x > 0 {
                potential_neighbours.push(Point2D { x: x - 1, y })
            }
            if x < height_map.num_cols {
                potential_neighbours.push(Point2D { x: x + 1, y })
            }
            if y > 0 {
                potential_neighbours.push(Point2D { x, y: y - 1 })
            }
            if y < height_map.num_rows {
                potential_neighbours.push(Point2D { x, y: y + 1 })
            }

            potential_neighbours
                .into_iter()
                .filter(|point| height_map.heights.get(&point).unwrap() - this_height < 1)
                .map(|point| (point, 1))
                .collect::<Vec<_>>()
        },
        |&point| 26 - height_map.heights.get(&point).unwrap(),
        |p| *p == height_map.goal,
    )
    .expect("there must be at least one shortest path");
    Ok(shortest_path.1)
}

pub fn p2(_file: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d12/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 31);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d12/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d12/test.txt").unwrap();
        assert_eq!(p2(&inp), 8);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d12/real.txt").unwrap();
        assert_eq!(p2(&inp), 0);
    }
}
