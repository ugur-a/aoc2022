use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use regex::Regex;

struct Border {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}

struct Point2D(i32, i32);

struct Sensor {
    coords: Point2D,
    nearest_beacon_coords: Point2D,
}

struct Map {
    sensors: Vec<Sensor>,
    borders: Border,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let coords_regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )?;
        let sensors = s
            .lines()
            .map(|line| {
                coords_regex
                    .captures_iter(line)
                    .map(|caps| caps.extract().1.map(|coord| i32::from_str(coord).unwrap()))
                    .exactly_one()
                    .unwrap()
            })
            .map(|[sensor_x, sensor_y, beacon_x, beacon_y]| Sensor {
                coords: Point2D(sensor_x, sensor_y),
                nearest_beacon_coords: Point2D(beacon_x, beacon_y),
            })
            .collect::<Vec<_>>();

        let borders = {
            let itertools::MinMaxResult::MinMax(left, right) = sensors
                .iter()
                .flat_map(
                    |Sensor {
                         coords,
                         nearest_beacon_coords,
                     }| [coords.0, nearest_beacon_coords.0],
                )
                .minmax()
            else {
                unreachable!()
            };

            let itertools::MinMaxResult::MinMax(up, down) = sensors
                .iter()
                .flat_map(
                    |Sensor {
                         coords,
                         nearest_beacon_coords,
                     }| [coords.1, nearest_beacon_coords.1],
                )
                .minmax()
            else {
                unreachable!()
            };

            Border {
                left,
                right,
                up,
                down,
            }
        };

        Ok(Self { sensors, borders })
    }
}

pub fn p1(file: &str, row_to_analyze: usize) -> Result<usize> {
    todo!()
}
pub fn p2(_file: &str) -> Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d14/test.txt").unwrap();
        assert_eq!(p1(&inp, 10).unwrap(), 26);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p1(&inp, 2000000).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d14/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d14/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
