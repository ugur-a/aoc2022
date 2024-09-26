use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use nom::{
    character::complete::{char, i8, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use pathfinding::directed::dfs::dfs_reach;

use libaoc::{impl_from_str_from_nom_parser, points::Point3D};

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
        let (x_min, x_max) = self.cubes().map(Point3D::x).minmax().into_option().unwrap();
        let (y_min, y_max) = self.cubes().map(Point3D::y).minmax().into_option().unwrap();
        let (z_min, z_max) = self.cubes().map(Point3D::z).minmax().into_option().unwrap();
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

impl_from_str_from_nom_parser!(droplet, Droplet);

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
                .filter(|&point| !(droplet.contains(point)))
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
        .filter(|&(potentially_exposed_side, _)| !(droplet.contains(potentially_exposed_side)))
        .filter(|&(_, num_neighbours)| num_neighbours < 6)
        .filter(|(exposed_side, _)| exteriour_sides.contains(exposed_side))
        .map(|(_, num_neighbours)| num_neighbours)
        .sum();

    Ok(num_exteriour_exposed_sides)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 64)]
    #[test_case(REAL => 3526)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 58)]
    #[test_case(REAL => 2090)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
