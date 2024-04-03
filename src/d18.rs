use std::collections::HashSet;

use anyhow::bail;
use itertools::{Itertools, MinMaxResult};
use pathfinding::directed::dfs::dfs_reach;

use crate::points::Point3D;

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
    fn new(cubes: HashSet<DropletCube>) -> Self {
        Self { cubes }
    }

    fn cubes(&self) -> impl Iterator<Item = &DropletCube> {
        self.cubes.iter()
    }

    fn contains(&self, cube: Point3D<i8>) -> bool {
        self.cubes.contains(&cube)
    }

    fn boundaries(&self) -> anyhow::Result<DropletBoundaries> {
        let MinMaxResult::MinMax(x_min, x_max) = self.cubes().map(Point3D::x).minmax() else {
            bail!("cube unbound in x axis")
        };
        let MinMaxResult::MinMax(y_min, y_max) = self.cubes().map(Point3D::y).minmax() else {
            bail!("cube unbound in y axis")
        };
        let MinMaxResult::MinMax(z_min, z_max) = self.cubes().map(Point3D::z).minmax() else {
            bail!("cube unbound in z axis")
        };
        Ok(DropletBoundaries {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        })
    }
}

fn parse_droplet(s: &str) -> Result<DropletCube> {
    let [x, y, z] = s
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?[..]
    else {
        bail!("num coords of a droplet != 3")
    };
    Ok(Point3D(x, y, z))
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    let droplet_cubes: HashSet<DropletCube> =
        file.lines().map(parse_droplet).collect::<Result<_>>()?;
    let droplet = Droplet::new(droplet_cubes);

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
    let droplet_cubes: HashSet<DropletCube> =
        file.lines().map(parse_droplet).collect::<Result<_>>()?;
    let droplet = Droplet::new(droplet_cubes);

    let boundaries = droplet.boundaries()?;
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

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/d18/test.txt");
        assert_eq!(p1(inp).unwrap(), 64);
    }
    #[test]
    fn real_p1() {
        let inp = include_str!("../inputs/d18/real.txt");
        assert_eq!(p1(inp).unwrap(), 3526);
    }
    #[test]
    fn test_p2() {
        let inp = include_str!("../inputs/d18/test.txt");
        assert_eq!(p2(inp).unwrap(), 58);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = include_str!("../inputs/d18/real.txt");
        assert_eq!(p2(inp).unwrap(), 2090);
    }
}
