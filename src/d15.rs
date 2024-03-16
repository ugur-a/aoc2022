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

impl Point2D {
    fn manhattan_distance(&self, other: &Point2D) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

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

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn p1(file: &str, analyzed_row_num: i32) -> Result<usize> {
    let map = Map::from_str(file)?;

    let mut impossible_locations_of_distress_beacon =
        HashSet::with_capacity((map.borders.right - map.borders.left) as usize);

    for (signal, beacon) in &map.sensors_with_beacons {
        let distance_to_beacon = signal.manhattan_distance(&beacon);
        let distance_to_analyzed_row = signal.1.abs_diff(analyzed_row_num);

        match distance_to_analyzed_row.cmp(&distance_to_beacon) {
            std::cmp::Ordering::Greater => continue,
            std::cmp::Ordering::Equal => {
                impossible_locations_of_distress_beacon.insert(signal.0);
            }
            std::cmp::Ordering::Less => {
                let width_of_covered_space_on_the_analyzed_row =
                    distance_to_beacon - distance_to_analyzed_row;

                let leftmost_impossible_location =
                    signal.0 - width_of_covered_space_on_the_analyzed_row as i32;
                let rightmost_impossible_location =
                    signal.0 + width_of_covered_space_on_the_analyzed_row as i32;

                impossible_locations_of_distress_beacon
                    .extend(leftmost_impossible_location..=rightmost_impossible_location);
            }
        }
    }

    // "is `x=2,y=10` a "position where a beacon cannot be present"?"
    for beacon in map.sensors_with_beacons.values() {
        if beacon.1 == analyzed_row_num {
            impossible_locations_of_distress_beacon.remove(&beacon.0);
        }
    }

    Ok(impossible_locations_of_distress_beacon.len())
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
        let inp = read_to_string("inputs/d15/test.txt").unwrap();
        assert_eq!(p1(&inp, 10).unwrap(), 26);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d15/real.txt").unwrap();
        assert_eq!(p1(&inp, 2_000_000).unwrap(), 4748135);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d15/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d15/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
