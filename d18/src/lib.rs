use std::{collections::HashSet, str::FromStr};

use itertools::{Itertools, MinMaxResult};
use nom::{
    character::complete::{char, i8, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use pathfinding::directed::dfs::dfs_reach;

use aoc2022lib::points::Point3D;

type DropletCube = Point3D<i8>;

struct DropletBoundaries {
    x_min: i8,
    x_max: i8,
    y_min: i8,
    y_max: i8,
    z_min: i8,
    z_max: i8,
}

struct Droplet {
    cubes: HashSet<DropletCube>,
}

impl Droplet {
    fn from_droplet_cubes<I: IntoIterator<Item = DropletCube>>(cubes: I) -> Self {
        Self {
            cubes: HashSet::from_iter(cubes),
        }
    }

    fn cubes(&self) -> impl Iterator<Item = &DropletCube> {
        self.cubes.iter()
    }

    fn contains(&self, cube: Point3D<i8>) -> bool {
        self.cubes.contains(&cube)
    }

    fn boundaries(&self) -> DropletBoundaries {
        let (x_min, x_max) = match self.cubes().map(Point3D::x).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(only) => (only, only),
            MinMaxResult::NoElements => unreachable!(),
        };

        let (y_min, y_max) = match self.cubes().map(Point3D::y).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(only) => (only, only),
            MinMaxResult::NoElements => unreachable!(),
        };

        let (z_min, z_max) = match self.cubes().map(Point3D::z).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(only) => (only, only),
            MinMaxResult::NoElements => unreachable!(),
        };

        DropletBoundaries {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}

// 4,3,2
fn droplet_cube(input: &str) -> IResult<&str, DropletCube> {
    map(
        tuple((i8, preceded(char(','), i8), preceded(char(','), i8))),
        |(x, y, z)| Point3D(x, y, z),
    )(input)
}

fn droplet(input: &str) -> IResult<&str, Droplet> {
    map(
        separated_list1(newline, droplet_cube),
        Droplet::from_droplet_cubes,
    )(input)
}

impl FromStr for Droplet {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match droplet(s).finish() {
            Ok((_remaining, droplet)) => Ok(droplet),
            Err(nom::error::Error { input, code }) => Err(Self::Err {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let droplet = Droplet::from_str(file)?;

    // multiple droplets can have the same point as a potential exposed side (PES),
    // so there will be duplicate values here
    let num_exposed_sides: usize = droplet
        .cubes()
        .flat_map(Point3D::neighbours)
        .filter(|&potentially_exposed_side| !(droplet.contains(potentially_exposed_side)))
        .count();

    Ok(num_exposed_sides)
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let droplet = Droplet::from_str(file)?;

    let boundaries = droplet.boundaries();
    // sides accessible from outside the droplet
    let exteriour_sides: HashSet<Point3D<i8>> = dfs_reach(
        Point3D::<i8>(boundaries.x_min, boundaries.y_min, boundaries.z_min),
        |air_point: &Point3D<i8>| {
            air_point
                .neighbours()
                .into_iter()
                // can't go inside droplet
                .filter(|point| !(droplet.contains(*point)))
                // limit the searched volume to around the droplet
                .filter(|Point3D(x, y, z)| {
                    ((boundaries.x_min - 1)..=(boundaries.x_max + 1)).contains(x)
                        && ((boundaries.y_min - 1)..=(boundaries.y_max + 1)).contains(y)
                        && ((boundaries.z_min - 1)..=(boundaries.z_max + 1)).contains(z)
                })
        },
    )
    .collect();

    let num_exteriour_exposed_sides = droplet
        .cubes()
        .flat_map(Point3D::neighbours)
        // multiple droplets can have the same point as a potential exposed side (PES),
        // so count occurences of each value
        .counts()
        .into_iter()
        .filter(|(potentially_exposed_side, _num_neighbours)| {
            !(droplet.contains(*potentially_exposed_side))
        })
        .filter(|(_exposed_side, num_neighbours)| *num_neighbours < 6)
        .filter(|(exposed_side, _num_neighbours)| exteriour_sides.contains(exposed_side))
        .map(|(_exteriour_exposed_sides, num_neighbours)| num_neighbours)
        .sum();

    Ok(num_exteriour_exposed_sides)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 64);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3526);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 58);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 2090);
    }
}
