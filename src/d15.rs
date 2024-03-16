use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{Error, Result};
use itertools::Itertools;
use regex::Regex;

struct Border {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}

#[derive(PartialEq, Eq, Hash)]
struct Point2D(i32, i32);

type SensorPosition = Point2D;
type BeaconPosition = Point2D;

struct Map {
    sensors_with_beacons: HashMap<SensorPosition, BeaconPosition>,
    borders: Border,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let coords_regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )?;
        let sensors_with_beacons = s
            .lines()
            .map(|line| {
                coords_regex
                    .captures_iter(line)
                    .map(|caps| caps.extract().1.map(|coord| i32::from_str(coord).unwrap()))
                    .exactly_one()
                    .unwrap()
            })
            .map(|[sensor_x, sensor_y, beacon_x, beacon_y]| {
                (Point2D(sensor_x, sensor_y), Point2D(beacon_x, beacon_y))
            })
            .collect::<HashMap<_, _>>();

        let borders = {
            let itertools::MinMaxResult::MinMax(left, right) = sensors_with_beacons
                .iter()
                .flat_map(|(coords, nearest_beacon_coords)| [coords.0, nearest_beacon_coords.0])
                .minmax()
            else {
                unreachable!()
            };

            let itertools::MinMaxResult::MinMax(up, down) = sensors_with_beacons
                .iter()
                .flat_map(|(coords, nearest_beacon_coords)| [coords.1, nearest_beacon_coords.1])
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

        Ok(Self {
            sensors_with_beacons,
            borders,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for row in self.borders.up..=self.borders.down {
            for col in self.borders.left..=self.borders.right {
                let point = Point2D(col, row);
                let chr = {
                    if self.sensors_with_beacons.contains_key(&point) {
                        'S'
                    } else if self.sensors_with_beacons.values().contains(&point) {
                        'B'
                    } else {
                        '.'
                    }
                };
                res.push(chr);
            }
            res.push('\n');
        }
        write!(f, "{res}")
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
