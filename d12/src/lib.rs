use std::{collections::HashMap, str::FromStr};

use anyhow::Context;
use aoc2022lib::points::Point2D;
use pathfinding::directed::astar;

struct HeightMap<T> {
    start: Point2D<T>,
    goal: Point2D<T>,
    num_rows: usize,
    num_cols: usize,
    heights: HashMap<Point2D<T>, u32>,
}

impl HeightMap<usize> {
    fn climbable_neighbours(&self, point: Point2D<usize>) -> Vec<Point2D<usize>> {
        let Point2D(x, y) = point;
        let this_height = self.heights[&point];

        let mut potential_neighbours = Vec::new();
        if x > 0 {
            potential_neighbours.push(Point2D(x - 1, y));
        }
        if x < self.num_cols - 1 {
            potential_neighbours.push(Point2D(x + 1, y));
        }
        if y > 0 {
            potential_neighbours.push(Point2D(x, y - 1));
        }
        if y < self.num_rows - 1 {
            potential_neighbours.push(Point2D(x, y + 1));
        }

        potential_neighbours
            .into_iter()
            .filter(|point| self.heights[point] <= this_height + 1)
            .collect()
    }
}

impl FromStr for HeightMap<usize> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_row, s_col, _s_char) = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, line)| {
                line.char_indices()
                    .map(move |(col_num, char)| (row_num, col_num, char))
            })
            .find(|(_row_num, _col_num, char)| *char == 'S')
            .with_context(|| "no starting point found")?;
        let start = Point2D(s_col, s_row);

        let (g_row, g_col, _g_char) = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, line)| {
                line.char_indices()
                    .map(move |(col_num, char)| (row_num, col_num, char))
            })
            .find(|(_row_num, _col_num, char)| *char == 'E')
            .with_context(|| "no end point found")?;
        let goal = Point2D(g_col, g_row);

        let num_cols = s.lines().next().with_context(|| "map has no rows")?.len();

        let num_rows = s.lines().count();

        let heights: HashMap<Point2D<usize>, u32> = s
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
                    .map(move |(col_num, height)| (Point2D(col_num, row_num), height))
            })
            .collect();
        Ok(Self {
            start,
            goal,
            num_rows,
            num_cols,
            heights,
        })
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let height_map = HeightMap::from_str(file)?;
    let (_, shortest_path) = astar::astar(
        &height_map.start,
        |&point| {
            height_map
                .climbable_neighbours(point)
                .into_iter()
                .map(|point| (point, 1))
        },
        |point| 26 - height_map.heights.get(point).unwrap(),
        |&point| point == height_map.goal,
    )
    .context("there must be at least one shortest path")?;
    Ok(shortest_path)
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let height_map = HeightMap::from_str(file)?;
    height_map
        .heights
        .iter()
        .filter(|&(_point, height)| *height == 0)
        .filter_map(|(lowest_point, _height)| {
            astar::astar(
                lowest_point,
                |&point| {
                    height_map
                        .climbable_neighbours(point)
                        .into_iter()
                        .map(|point| (point, 1))
                },
                |point| 26 - height_map.heights[point],
                |&point| point == height_map.goal,
            )
        })
        .map(|(_path, path_length)| path_length)
        .min()
        .context("there must be at least one shortest path")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 31);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 370);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 29);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 363);
    }
}
