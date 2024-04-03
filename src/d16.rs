use std::{collections::HashMap, iter::repeat};

use petgraph::graphmap::UnGraphMap;
use regex::Regex;

#[derive(Clone, Copy)]
struct Valve {
    flow_rate: u32,
    is_opened: bool,
}

impl Valve {
    fn new(flow_rate: u32) -> Self {
        Self {
            flow_rate,
            is_opened: false,
        }
    }

    fn is_opened(&self) -> bool {
        self.is_opened
    }

    fn open(&mut self) {
        self.is_opened = true;
    }
}

struct Network<'a> {
    valves: HashMap<&'a str, Valve>,
    tunnel_graph: UnGraphMap<&'a str, u32>,
}

fn parse_network(s: &str) -> anyhow::Result<Network> {
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves ((?:[A-Z]{2}, )*[A-Z]{2})",
    )?;
    let initially_parsed_input: Vec<(&str, u32, Vec<&str>)> = s
        .lines()
        .map(|line| re.captures(line).unwrap().extract::<3>().1)
        .map(|[valve, flow_rate, neighbours]| {
            (
                valve,
                flow_rate.parse::<u32>().unwrap(),
                neighbours.split(", ").collect::<Vec<_>>(),
            )
        })
        .collect();

    let valves: HashMap<&str, Valve> = initially_parsed_input
        .iter()
        .map(|(valve, flow_rate, _neighbours)| (*valve, Valve::new(*flow_rate)))
        .collect();

    let tunnel_graph: UnGraphMap<&str, u32> = initially_parsed_input
        .iter()
        .flat_map(|(valve, _flow_rate, neighbours)| repeat(valve).zip(neighbours))
        .fold(
            UnGraphMap::with_capacity(s.lines().count(), 0),
            |mut tunnels_graph, (valve1, valve2)| {
                tunnels_graph.add_edge(&valve1, &valve2, 1);
                tunnels_graph
            },
        );

    Ok(Network {
        valves,
        tunnel_graph,
    })
}

pub fn p1(file: &str) -> anyhow::Result<u32> {
    todo!()
}
pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/d16/test.txt");
        assert_eq!(p1(&inp).unwrap(), 1651);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = include_str!("../inputs/d16/real.txt");
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = include_str!("../inputs/d16/test.txt");
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = include_str!("../inputs/d16/real.txt");
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
