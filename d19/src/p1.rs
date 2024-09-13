use std::str::FromStr;

use crate::{
    bp::Blueprint,
    optimizer::{BlueprintOptimizer, Quality},
};
use grb::prelude::*;

const TIME_LIMIT: usize = 24;

struct P1;
impl<const N: usize> Quality for BlueprintOptimizer<N, P1> {
    fn quality(mut self) -> u32 {
        let num_scenarios = self.model.get_attr(attr::NumScenarios).unwrap();

        (0..num_scenarios)
            .map(|n| {
                self.model.set_param(param::ScenarioNumber, n).unwrap();
                let quality: u32 = {
                    let quality = self.model.get_attr(attr::ScenNObjVal).unwrap();
                    assert!(quality.is_finite());
                    // safety: checked in previous line
                    unsafe { quality.to_int_unchecked() }
                };

                (n + 1) as u32 * quality
            })
            .sum()
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let mut optimizer = BlueprintOptimizer::<TIME_LIMIT, P1>::new(file.lines().count());

    for line in file.lines() {
        let bp = Blueprint::from_str(line)?;
        optimizer.add_bp(bp);
    }
    optimizer.optimize()?;
    Ok(optimizer.quality())
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
