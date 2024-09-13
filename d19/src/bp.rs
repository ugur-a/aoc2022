use aoc2022lib::impl_from_str_from_nom_parser;
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    combinator::map,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

fn ore(i: &str) -> IResult<&str, u32> {
    terminated(u32, tag(" ore"))(i)
}
fn clay(i: &str) -> IResult<&str, u32> {
    terminated(u32, tag(" clay"))(i)
}
fn obsidian(i: &str) -> IResult<&str, u32> {
    terminated(u32, tag(" obsidian"))(i)
}

struct OreRobot {
    cost_ore: u32,
}

struct ClayRobot {
    cost_ore: u32,
}

struct ObsidianRobot {
    cost_ore: u32,
    cost_clay: u32,
}

struct GeodeRobot {
    cost_ore: u32,
    cost_obs: u32,
}
// Each ore robot costs 4 ore.
fn ore_robot(i: &str) -> IResult<&str, OreRobot> {
    map(
        delimited(tag(" Each ore robot costs "), ore, tag(".")),
        |cost_ore| OreRobot { cost_ore },
    )(i)
}
fn clay_robot(i: &str) -> IResult<&str, ClayRobot> {
    map(
        delimited(tag(" Each clay robot costs "), ore, tag(".")),
        |cost_ore| ClayRobot { cost_ore },
    )(i)
}
fn obs_robot(i: &str) -> IResult<&str, ObsidianRobot> {
    map(
        delimited(
            tag(" Each obsidian robot costs "),
            separated_pair(ore, tag(" and "), clay),
            tag("."),
        ),
        |(cost_ore, cost_clay)| ObsidianRobot {
            cost_ore,
            cost_clay,
        },
    )(i)
}
fn geode_robot(i: &str) -> IResult<&str, GeodeRobot> {
    map(
        delimited(
            tag(" Each geode robot costs "),
            separated_pair(ore, tag(" and "), obsidian),
            tag("."),
        ),
        |(cost_ore, cost_obs)| GeodeRobot { cost_ore, cost_obs },
    )(i)
}

pub(crate) struct Blueprint {
    pub id: u32,
    ore_robot: OreRobot,
    clay_robot: ClayRobot,
    obs_robot: ObsidianRobot,
    geode_robot: GeodeRobot,
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
fn blueprint(i: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            delimited(tag("Blueprint "), u32, tag(":")),
            ore_robot,
            clay_robot,
            obs_robot,
            geode_robot,
        )),
        |(id, ore_robot, clay_robot, obs_robot, geode_robot)| Blueprint {
            id,
            ore_robot,
            clay_robot,
            obs_robot,
            geode_robot,
        },
    )(i)
}

impl_from_str_from_nom_parser!(blueprint, Blueprint);

impl Blueprint {
    pub(crate) fn into_robot_costs(self) -> [[u32; 4]; 4] {
        [
            [self.ore_robot.cost_ore, 0, 0, 0],
            [self.clay_robot.cost_ore, 0, 0, 0],
            [self.obs_robot.cost_ore, self.obs_robot.cost_clay, 0, 0],
            [self.geode_robot.cost_ore, 0, self.geode_robot.cost_obs, 0],
        ]
    }
}
