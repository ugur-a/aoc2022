use std::str::FromStr;

use crate::{
    bp::Blueprint,
    optimizer::{BlueprintOptimizer, Quality},
};
use grb::prelude::*;

const TIME_LIMIT: usize = 32;
const MAX_NUM_BLUEPRINTS: usize = 3;

struct P2;
impl<const N: usize> Quality for BlueprintOptimizer<N, P2> {
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

                quality
            })
            .product()
    }
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    let num_blueprints = std::cmp::min(MAX_NUM_BLUEPRINTS, file.lines().count());
    let mut optimizer = BlueprintOptimizer::<TIME_LIMIT, P2>::new(num_blueprints);

    for line in file.lines().take(num_blueprints) {
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
        assert_eq!(p2(&inp).unwrap(), 56 * 62);
    }

    #[test]
    fn real() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 4400);
    }
}
