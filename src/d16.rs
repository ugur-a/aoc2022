use std::{collections::HashMap, iter::repeat};

use anyhow::Result;
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


pub fn p1(file: &str) -> Result<u32> {
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
        let inp = read_to_string("inputs/d16/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 1651);
    }
    #[test]
    #[ignore]
    fn real_p1() {
        let inp = read_to_string("inputs/d16/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d16/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d16/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
