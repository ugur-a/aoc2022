use std::str::FromStr;

use crate::{bp::Blueprint, optimizer::BlueprintOptimizer};

const TIME_LIMIT: usize = 24;

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let mut optimizer = BlueprintOptimizer::<TIME_LIMIT>::new(file.lines().count());

    for line in file.lines() {
        let bp = Blueprint::from_str(line)?;
        optimizer.add_bp(bp);
    }
    optimizer.total_quality()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example() {
        let inp = read_to_string("inputs/example.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 33);
    }

    #[test]
    fn real() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 1427);
    }
}
