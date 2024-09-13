// caused by grb::c
#![allow(clippy::useless_conversion)]
use core::array;
use std::{iter::zip, marker::PhantomData};

use grb::prelude::*;

use crate::bp::Blueprint;

pub(crate) struct BlueprintOptimizer<const TIME_LIMIT: usize, P> {
    _marker: PhantomData<P>,
    pub(crate) model: Model,
    pub(crate) robots_costs: [[Var; 4]; 4],
}

fn kmrm(
    kind: &str,
    minute: Option<usize>,
    robot: Option<usize>,
    material: Option<usize>,
) -> String {
    std::iter::once(String::from(kind))
        .chain(minute.map(|m| format!("_min{}", m + 1)))
        .chain(robot.map(|r| format!("_rob{r}")))
        .chain(material.map(|m| format!("_mat{m}")))
        .collect()
}

impl<const TIME_LIMIT: usize, P> BlueprintOptimizer<TIME_LIMIT, P> {
    #[allow(clippy::useless_conversion)]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::needless_range_loop)]
    pub(crate) fn new(num_scenarios: usize) -> Self {
        const STARTING_ROBOTS: [u32; 4] = [1, 0, 0, 0];
        // after minute 1
        const STARTING_MATERIALS: [u32; 4] = [1, 0, 0, 0];

        let env = Env::empty().unwrap();
        // env.set(param::OutputFlag, 0).unwrap();
        let env = env.start().unwrap();
        let mut model = Model::with_env("Model", env).unwrap();

        model
            .set_attr(attr::NumScenarios, num_scenarios as i32)
            .unwrap();

        // the ith element stores amounts at the _end_ of i+1th minute
        let materials: [[Var; 4]; TIME_LIMIT] = array::from_fn(|minute| {
            array::from_fn(|material| {
                add_intvar!(model, name: &kmrm("material", Some(minute), None, Some(material)))
                    .unwrap()
            })
        });
        let robots_built: [[Var; 4]; TIME_LIMIT] = array::from_fn(|minute| {
            array::from_fn(|robot| {
                add_binvar!(model, name: &kmrm("robot_built", Some(minute), Some(robot), None))
                    .unwrap()
            })
        });
        let robots: [[Var; 4]; TIME_LIMIT] = array::from_fn(|minute| {
            array::from_fn(|robot| {
                add_intvar!(model, name: &kmrm("robot", Some(minute), Some(robot), None)).unwrap()
            })
        });
        let robots_costs: [[Var; 4]; 4] = array::from_fn(|robot| {
            array::from_fn(|material| {
                add_intvar!(model, name: &kmrm("robot_cost", None, Some(robot), Some(material)))
                    .unwrap()
            })
        });
        let building_costs: [[Var; 4]; TIME_LIMIT] = array::from_fn(|minute| {
            array::from_fn(|material| {
                add_intvar!(model, name: &kmrm("building_costs", Some(minute), None, Some(material))).unwrap()
            })
        });

        // add all the variables
        // NOTE: without this `robot_built` later fails
        model.update().unwrap();

        for robot in 0..4 {
            model
                .add_constr(
                    &kmrm("starting_robots", Some(0), Some(robot), None),
                    c!(robots[0][robot] == STARTING_ROBOTS[robot]),
                )
                .unwrap();
        }

        for material in 0..4 {
            model
                .add_constr(
                    &kmrm("starting_materials", Some(0), None, Some(material)),
                    c!(materials[0][material] == STARTING_MATERIALS[material]),
                )
                .unwrap();
        }

        for minute in 0..TIME_LIMIT - 1 {
            // rs_end_3 = rs_start_3 + rbs_2 = rs_end_2 + rbs_2
            for robot in 0..4 {
                model
                    .add_constr(
                        &kmrm("robots_next_minute", Some(minute), Some(robot), None),
                        c!(robots[minute + 1][robot]
                            == robots[minute][robot] + robots_built[minute][robot]),
                    )
                    .unwrap();
            }

            // ms_end_3 = ms_start_3 - rcs_2 + rs_start_3 = ms_end_2 - rcs_2 + rs_end_2
            for material in 0..4 {
                model
                    .add_constr(
                        &kmrm("materials_next_minute", Some(minute), None, Some(material)),
                        c!(materials[minute + 1][material]
                            == materials[minute][material] - building_costs[minute][material]
                                + robots[minute][material]),
                    )
                    .unwrap();
            }
        }

        for minute in 0..TIME_LIMIT {
            for robot in 0..4 {
                // rbs_3 == 1 => ms_start_3 >= rcs_2 <=> ms_end_2 => rcs_2
                for material in 0..4 {
                    // whether can build robot, based on availability of each material
                    model
                        .add_genconstr_indicator(
                            &kmrm("robot_built", Some(minute), Some(robot), Some(material)),
                            robots_built[minute][robot],
                            true,
                            c!(robots_costs[robot][material] <= materials[minute][material]),
                        )
                        .unwrap();
                }
            }

            model
                .add_constr(
                    &kmrm("max_1_robot_per_minute", Some(minute), None, None),
                    c!(robots_built[minute].grb_sum() <= 1),
                )
                .unwrap();

            for material in 0..4 {
                model
                    .add_qconstr(
                        // total cost of building in any minute is
                        // building cost of the robot built in that minute
                        &kmrm("building_costs", Some(minute), None, Some(material)),
                        c!(building_costs[minute][material]
                            == robots_costs[0][material] * robots_built[minute][0]
                                + robots_costs[1][material] * robots_built[minute][1]
                                + robots_costs[2][material] * robots_built[minute][2]
                                + robots_costs[3][material] * robots_built[minute][3]),
                    )
                    .unwrap();
            }
        }

        model
            .set_objective(materials[TIME_LIMIT - 1][3], grb::ModelSense::Maximize)
            .unwrap();

        model.write("model.lp").unwrap();
        Self {
            model,
            robots_costs,
            _marker: PhantomData,
        }
    }

    pub(crate) fn add_bp(&mut self, bp: Blueprint) {
        let BlueprintOptimizer {
            model,
            robots_costs,
            ..
        } = self;
        model
            .set_param(param::ScenarioNumber, bp.id as i32 - 1)
            .unwrap();

        let bp_robots_costs = bp.into_robot_costs();

        // bound each variable from both sides, effectively fixing its value
        for bound in [attr::ScenNLB, attr::ScenNUB] {
            model
                .set_obj_attr_batch(
                    bound,
                    zip(
                        (*robots_costs).into_iter().flatten(),
                        bp_robots_costs.into_iter().flatten().map(f64::from),
                    ),
                )
                .unwrap();
        }
    }

    pub(crate) fn optimize(&mut self) -> anyhow::Result<()> {
        Ok(self.model.optimize()?)
    }
}

pub(crate) trait Quality {
    fn quality(self) -> u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let _o = BlueprintOptimizer::<24, ()>::new(0);
    }
}
