use anyhow::{anyhow, ensure, Context};
use aoc2022lib::impl_from_str_for_obj_with_lifetimes_from_nom_parser;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use pathfinding::directed::dijkstra;
use petgraph::{algo::floyd_warshall, prelude::*};
use std::collections::HashMap;

struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    neighbours: Vec<&'a str>,
}

fn valve_name(i: &str) -> IResult<&str, &str> {
    map_res(alpha1, |s: &str| {
        ensure!(s.len() == 2 && s.chars().all(char::is_uppercase));
        Ok(s)
    })(i)
}

fn valve_name_singleton(i: &str) -> IResult<&str, Vec<&str>> {
    map(valve_name, |s| vec![s])(i)
}

fn valve_names(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), valve_name)(i)
}

// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn valve(i: &str) -> IResult<&str, Valve> {
    map(
        tuple((
            preceded(tag("Valve "), valve_name),
            preceded(tag(" has flow rate="), u32),
            alt((
                preceded(tag("; tunnels lead to valves "), valve_names),
                preceded(tag("; tunnel leads to valve "), valve_name_singleton),
            )),
        )),
        |(name, flow_rate, neighbours)| Valve {
            name,
            flow_rate,
            neighbours,
        },
    )(i)
}

impl_from_str_for_obj_with_lifetimes_from_nom_parser!(valve, Valve);

const START_VALVE: &str = "AA";
const TIME_LIMIT: u32 = 30;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    valve: &'a str,
    time: u32,
    closed_valves: Vec<&'a str>,
}

impl<'a> State<'a> {
    fn new(valve: &'a str, time: u32, closed_valves: Vec<&'a str>) -> Self {
        Self {
            valve,
            time,
            closed_valves,
        }
    }
    fn successors(
        &self,
        apsp: &HashMap<(&str, &str), u32>,
        valve_flows: &HashMap<&str, u32>,
    ) -> Vec<(Self, u32)> {
        let &Self {
            valve,
            time,
            ref closed_valves,
        }: &State<'a> = self;
        let pressure_opportunity_cost = closed_valves.iter().map(|cv| valve_flows[cv]).sum::<u32>();

        let res: Vec<_> = (0..closed_valves.len())
            .filter_map(|i| {
                let neighbour = closed_valves[i];

                // time to reach the valve _and_ open it
                let dtime = apsp[&(valve, neighbour)] + 1;

                if time + dtime > TIME_LIMIT {
                    return None;
                }

                let (neighbour, closed_valves_left) = {
                    let mut cv = closed_valves.clone();
                    let neighbour = cv.remove(i);
                    (neighbour, cv)
                };

                Some((
                    Self::new(neighbour, time + dtime, closed_valves_left),
                    dtime * pressure_opportunity_cost,
                ))
            })
            .collect();

        if res.is_empty() {
            let dtime = TIME_LIMIT - time;
            // can't reach anything, so just stay in place until the end
            // must include this successor, since this may be (and indeed is, in `real`)
            // a part of the optimal solution
            vec![(
                Self::new(valve, TIME_LIMIT, closed_valves.clone()),
                dtime * pressure_opportunity_cost,
            )]
        } else {
            res
        }
    }
    fn success(&self) -> bool {
        self.closed_valves.is_empty() || self.time == TIME_LIMIT
    }
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let valves: Vec<_> = file.lines().map(Valve::try_from).try_collect()?;

    let gr = UnGraphMap::<_, ()>::from_edges(valves.iter().flat_map(|v| {
        v.neighbours
            .iter()
            .map(move |&neighbour| (v.name, neighbour))
    }));

    let apsp = floyd_warshall(&gr, |_| 1u32).map_err(|_| anyhow!("Negative cycle"))?;

    let valve_flows: HashMap<_, _> = valves
        .into_iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| (v.name, v.flow_rate))
        .collect();

    let openable_valve_names: Vec<_> = valve_flows.keys().copied().collect();

    let (_path, total_pressure_unreleased) = dijkstra::dijkstra(
        &State::new(START_VALVE, 0, openable_valve_names),
        |state| state.successors(&apsp, &valve_flows),
        State::success,
    )
    .context("no path")?;

    let total_releasable_pressure = TIME_LIMIT * valve_flows.into_values().sum::<u32>();

    Ok(total_releasable_pressure - total_pressure_unreleased)
}

pub fn p2(file: &str) -> anyhow::Result<u32> {
    const START_TIME: u32 = 4;

    let valves: Vec<_> = file.lines().map(Valve::try_from).try_collect()?;

    let gr = UnGraphMap::<_, ()>::from_edges(valves.iter().flat_map(|v| {
        v.neighbours
            .iter()
            .map(move |&neighbour| (v.name, neighbour))
    }));

    let apsp = floyd_warshall(&gr, |_| 1u32).map_err(|_| anyhow!("Negative cycle"))?;

    let valve_flows: HashMap<_, _> = valves
        .into_iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| (v.name, v.flow_rate))
        .collect();

    let openable_valve_names: Vec<_> = valve_flows.keys().copied().collect();

    // it's always the best for each one to open half the valves
    // if #openable_valves is odd, can just consider the case where you open one less valve,
    // since opening one more valve will be exactly what the elephant does
    let n_valves_you = openable_valve_names.len() / 2;

    let total_pressure_unreleased = (openable_valve_names.iter().copied())
        .combinations(n_valves_you)
        .map(|you_valves| {
            let elep_valves = openable_valve_names
                .iter()
                .filter(|v| !you_valves.contains(v))
                .copied()
                .collect_vec();

            let total_pressure_unreleased: u32 = [you_valves, elep_valves]
                .into_iter()
                .map(|valves| {
                    let (_path, half_pressure_unreleased) = dijkstra::dijkstra(
                        &State::new(START_VALVE, START_TIME, valves),
                        |state| state.successors(&apsp, &valve_flows),
                        State::success,
                    )
                    .unwrap();

                    half_pressure_unreleased
                })
                .sum();
            total_pressure_unreleased
        })
        .min()
        .unwrap();

    let total_releasable_pressure =
        (TIME_LIMIT - START_TIME) * valve_flows.into_values().sum::<u32>();

    Ok(total_releasable_pressure - total_pressure_unreleased)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 1651);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 2119);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 1707);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 2615);
    }
}
