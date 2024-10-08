use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use libaoc::{
    impl_from_str_from_nom_parser,
    points::{ManhattanDistance, Point2D},
};

use anyhow::Context;
use derive_deref::Deref;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Point = Point2D<i32>;

// x=2, y=18
fn point(i: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), i32),
            tag(", "),
            preceded(tag("y="), i32),
        ),
        |(x, y)| Point2D(x, y),
    )(i)
}

type SensorPosition = Point;
type BeaconPosition = Point;

struct SensorWithBeacon(SensorPosition, BeaconPosition);

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn sensor_with_beacon(i: &str) -> IResult<&str, SensorWithBeacon> {
    map(
        separated_pair(
            preceded(tag("Sensor at "), point),
            tag(": "),
            preceded(tag("closest beacon is at "), point),
        ),
        |(s, b)| SensorWithBeacon(s, b),
    )(i)
}

impl_from_str_from_nom_parser!(sensor_with_beacon, SensorWithBeacon);

#[derive(Deref)]
struct SensorsWithBeacons(HashMap<SensorPosition, BeaconPosition>);

impl FromStr for SensorsWithBeacons {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors_with_beacons = s
            .lines()
            .map(SensorWithBeacon::from_str)
            .map_ok(|SensorWithBeacon(sensor_pos, beacon_pos)| (sensor_pos, beacon_pos))
            .try_collect()?;

        Ok(Self(sensors_with_beacons))
    }
}

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
    // use drain_filter when it's stable
    // https://github.com/rust-lang/rust/issues/43244
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors_with_distances: HashMap<Point2D<i32>, u32> = SensorsWithBeacons::from_str(s)?
            .par_iter()
            .map(|(sensor_coords, beacon_coords)| {
                let distance = sensor_coords.manhattan_distance(*beacon_coords);
                (*sensor_coords, distance)
            })
            .collect();

        Ok(Self(sensors_with_distances))
    }
}

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
        .filter(|&(x, y)| {
            0 <= x && x <= search_space_side_size && 0 <= y && y <= search_space_side_size
        })
        .map(Point2D::from)
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
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE, 10 => 26)]
    #[test_case(REAL, 2_000_000 => 4_748_135)]
    fn test_p1(inp: &str, analyzed_row_num: i32) -> usize {
        p1(inp, analyzed_row_num).unwrap()
    }
    #[test_case(EXAMPLE, 20 => 56_000_011)]
    #[test_case(REAL, 4_000_000 => 13_743_542_639_657)]
    fn test_p2(inp: &str, search_space_side_size: i32) -> u64 {
        p2(inp, search_space_side_size).unwrap()
    }
}
