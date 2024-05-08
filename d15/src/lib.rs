use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc2022lib::points::Point2D;

use anyhow::Context;
use derive_deref::Deref;
use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};
use regex::Regex;

trait ManhattanDistance {
    fn manhattan_distance(self, other: Self) -> u32;
}

impl ManhattanDistance for Point2D<i32> {
    fn manhattan_distance(self, other: Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

type SensorPosition = Point2D<i32>;
type BeaconPosition = Point2D<i32>;

#[derive(Deref)]
struct SensorsWithBeacons(HashMap<SensorPosition, BeaconPosition>);

impl FromStr for SensorsWithBeacons {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let coords_regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )?;
        let sensors_with_beacons = s
            .par_lines()
            .map(|line| coords_regex.captures(line).unwrap().extract().1)
            .map(|coords_pair| coords_pair.map(|coord| i32::from_str(coord).unwrap()))
            .map(|[sensor_x, sensor_y, beacon_x, beacon_y]| {
                (Point2D(sensor_x, sensor_y), Point2D(beacon_x, beacon_y))
            })
            .collect::<HashMap<_, _>>();

        Ok(Self(sensors_with_beacons))
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub fn p1(file: &str, analyzed_row_num: i32) -> anyhow::Result<usize> {
    let sensors_with_beacons = SensorsWithBeacons::from_str(file)?;

    let mut impossible_locations_of_distress_beacon: HashSet<i32> = sensors_with_beacons
        .par_iter()
        .filter_map(|(signal, beacon)| {
            let distance_to_beacon = signal.manhattan_distance(*beacon);
            let distance_to_analyzed_row = signal.1.abs_diff(analyzed_row_num);

            match distance_to_analyzed_row.cmp(&distance_to_beacon) {
                std::cmp::Ordering::Greater => None,
                std::cmp::Ordering::Equal => Some(signal.0..=signal.0),
                std::cmp::Ordering::Less => {
                    let width_of_covered_space_on_the_analyzed_row =
                        distance_to_beacon - distance_to_analyzed_row;

                    Some(
                        (signal.0 - width_of_covered_space_on_the_analyzed_row as i32)
                            ..=(signal.0 + width_of_covered_space_on_the_analyzed_row as i32),
                    )
                }
            }
        })
        .flatten()
        .collect();

    // "is `x=2,y=10` a "position where a beacon cannot be present"?"
    for beacon in sensors_with_beacons.values() {
        if beacon.1 == analyzed_row_num {
            impossible_locations_of_distress_beacon.remove(&beacon.0);
        }
    }

    Ok(impossible_locations_of_distress_beacon.len())
}

#[derive(Deref)]
struct SensorsWithDistances(HashMap<SensorPosition, u32>);

impl FromStr for SensorsWithDistances {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let sensors_with_distances = s
            .parse::<SensorsWithBeacons>()?
            .par_iter()
            .map(|(sensor_coords, beacon_coords)| {
                let distance = sensor_coords.manhattan_distance(*beacon_coords);
                (*sensor_coords, distance)
            })
            .collect::<HashMap<_, _>>();

        Ok(Self(sensors_with_distances))
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn p2(file: &str, search_space_side_size: i32) -> anyhow::Result<u64> {
    let sensors_with_distances = SensorsWithDistances::from_str(file)?;
    let distress_beacon = sensors_with_distances
        .par_iter()
        .flat_map(|(point, radius)| {
            // the vertices
            let radius = *radius as i32;
            let left = point.0 - radius - 1;
            let right = point.0 + radius + 1;
            let up = point.1 - radius - 1;
            let down = point.1 + radius + 1;
            // the sides (moving clockwise)
            let left_upper = (left..point.0).zip((up..point.1).rev());
            let right_upper = (point.0..right).zip(up..point.1);
            let right_lower = ((point.0..right).rev()).zip(point.1..down);
            let left_lower = ((left..point.0).rev()).zip((point.1..down).rev());

            left_upper
                .chain(right_upper)
                .chain(right_lower)
                .chain(left_lower)
                .collect_vec()
        })
        .filter(|(x, y)| {
            0 <= *x && *x <= search_space_side_size && 0 <= *y && *y <= search_space_side_size
        })
        .map(|(x, y)| Point2D(x, y))
        .find_any(|point| {
            sensors_with_distances
                .par_iter()
                .all(|(sensor, distance_to_nearest_beacon)| {
                    sensor.manhattan_distance(*point) > *distance_to_nearest_beacon
                })
        })
        .context("No distress beacon found")?;
    let tuning_frequency: u64 = 4_000_000u64 * distress_beacon.0 as u64 + distress_beacon.1 as u64;
    Ok(tuning_frequency)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp, 10).unwrap(), 26);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp, 2_000_000).unwrap(), 4_748_135);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp, 20).unwrap(), 56_000_011);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp, 4_000_000).unwrap(), 13_743_542_639_657);
    }
}
