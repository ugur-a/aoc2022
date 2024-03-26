use anyhow::{bail, Result};

use crate::points::Point3D;

type DropletCube = Point3D<i8>;

fn parse_droplet(s: &str) -> Result<DropletCube> {
    let [x, y, z] = s
        .split(',')
        .map(|coord| coord.parse::<i8>())
        .collect::<Result<Vec<_>, _>>()?[..]
    else {
        bail!("num coords of a droplet != 3")
    };
    Ok(Point3D(x, y, z))
}

pub fn p1(file: &str) -> Result<usize> {
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
        let inp = read_to_string("inputs/d18/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 64);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d18/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d18/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d18/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
