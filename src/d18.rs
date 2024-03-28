use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::points::Point3D;

type DropletCube = Point3D<i8>;

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

pub fn p1(file: &str) -> Result<usize> {
    let droplet_cubes: HashSet<DropletCube> =
        file.lines().map(parse_droplet).collect::<Result<_>>()?;

    // multiple droplets can have the same point as a potential exposed side (PES),
    // so there will be duplicate values here
    let num_exposed_sides: usize = droplet_cubes
        .iter()
        .flat_map(Point3D::get_neighbours)
        .filter(|&potentially_exposed_side| !(droplet_cubes.contains(&potentially_exposed_side)))
        .count();

    Ok(num_exposed_sides)
}

pub fn p2(_file: &str) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d18/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 64);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d18/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3526);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/d18/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 58);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d18/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
