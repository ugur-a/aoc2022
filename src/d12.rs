use std::str::FromStr;

use anyhow::{Context, Error, Result};

struct HeightPoint {
    coords: Point2D,
    height: u32,
}

#[derive(Debug, Default, Clone, Copy)]
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
    heights: Vec<HeightPoint>,
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
        let start = Point2D { x: s_row, y: s_col };

        let (g_row, g_col, _g_char) = s
            .lines()
            .enumerate()
            .flat_map(|(row_num, line)| {
                line.char_indices()
                    .map(move |(col_num, char)| (row_num, col_num, char))
            })
            .find(|(_row_num, _col_num, char)| *char == 'E')
            .context("no end point found")?;
        let goal = Point2D { x: g_row, y: g_col };

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
                    .map(move |(col_num, height)| HeightPoint {
                        coords: Point2D {
                            x: row_num,
                            y: col_num,
                        },
                        height,
                    })
            })
            .collect::<Vec<_>>();
        Ok(Self {
            start,
            goal,
            heights,
        })
    }
}

pub fn p1(file: &str) -> Result<u32> {
    let height_map = file.parse::<HeightMap>()?;
    todo!()
}
pub fn p2(file: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d13/test.txt").unwrap();
        assert_eq!(p1(&inp), 21);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d13/real.txt").unwrap();
        assert_eq!(p1(&inp), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d13/test.txt").unwrap();
        assert_eq!(p2(&inp), 8);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d13/real.txt").unwrap();
        assert_eq!(p2(&inp), 0);
    }
}
