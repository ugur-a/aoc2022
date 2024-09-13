use std::str::FromStr;

use anyhow::Context;
use aoc2022lib::{impl_from_str_from_nom_parser, parse::n};
use derive_deref::Deref;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}
struct Rearrangement {
    num_crates_to_move: usize,
    stack_to_move_to: usize,
    stack_to_take_from: usize,
}

fn stack_idx(i: &str) -> IResult<&str, usize> {
    map(n, |stack_idx: usize| stack_idx - 1)(i)
}

fn num_crates(i: &str) -> IResult<&str, usize> {
    n(i)
}

// move 15 from 6 to 4
fn rearrangement(i: &str) -> IResult<&str, Rearrangement> {
    map(
        tuple((
            preceded(tag("move "), num_crates),
            preceded(tag(" from "), stack_idx),
            preceded(tag(" to "), stack_idx),
        )),
        |(num_crates_to_move, stack_to_take_from, stack_to_move_to)| Rearrangement {
            num_crates_to_move,
            stack_to_move_to,
            stack_to_take_from,
        },
    )(i)
}

impl_from_str_from_nom_parser!(rearrangement, Rearrangement);

struct Crate {
    name: char,
}

// [F]
fn some_crate(i: &str) -> IResult<&str, Option<Crate>> {
    map(delimited(char('['), anychar, char(']')), |name| {
        Some(Crate { name })
    })(i)
}

fn no_crate(i: &str) -> IResult<&str, Option<Crate>> {
    map(tag("   "), |_| None)(i)
}

fn optional_crate(i: &str) -> IResult<&str, Option<Crate>> {
    alt((some_crate, no_crate))(i)
}

#[derive(Deref)]
struct CrateLine(Vec<Option<Crate>>);

fn crate_line(i: &str) -> IResult<&str, CrateLine> {
    map(separated_list0(char(' '), optional_crate), CrateLine)(i)
}

impl_from_str_from_nom_parser!(crate_line, CrateLine);

type Warehouse = Vec<Vec<char>>;

fn warehouse(s: &str) -> anyhow::Result<Warehouse> {
    // remove the last row of the stack arrangement schema - the one with stack numbers
    let (initial_stack_arrangement, last_row_of_stack_arrangement) =
        s.rsplit_once('\n').context("No stack numbers row")?;

    // since we don't need the last row anyway, use it to indirectly calculate the number of stacks
    let num_stacks = (last_row_of_stack_arrangement.len() + 1) / 4;

    // initialize the warehouse (collection of stacks)
    let mut stacks: Vec<Vec<char>> =
        vec![Vec::with_capacity(initial_stack_arrangement.lines().count()); num_stacks];

    // parse the initial stack arrangement - fill up the warehouse
    // comment: go over lines bottom-up, since that's how the crates are stacked
    for line in initial_stack_arrangement.lines().rev() {
        CrateLine::from_str(line)?
            .iter()
            // provide the stack number for each maybe-crate
            .enumerate()
            // if there's a crate, add it to the corresponding stack, skip if only air
            .for_each(|(idx, optional_crate)| {
                if let Some(some_crate) = optional_crate {
                    stacks[idx].push(some_crate.name);
                }
            });
    }
    Ok(stacks)
}

fn apply_rearrangement(
    warehouse: &mut Warehouse,
    rearrangement: &Rearrangement,
    crane_model: &CraneModel,
) {
    let current_length_of_stack_to_move_from = warehouse[rearrangement.stack_to_take_from].len();

    let crates_to_move = {
        let crates = warehouse[rearrangement.stack_to_take_from]
            .drain((current_length_of_stack_to_move_from - rearrangement.num_crates_to_move)..);

        match crane_model {
            CraneModel::CrateMover9000 => crates.rev().collect_vec(),
            CraneModel::CrateMover9001 => crates.collect(),
        }
    };

    warehouse[rearrangement.stack_to_move_to].extend(crates_to_move);
}

fn crates_at_the_top(warehouse: &Warehouse) -> anyhow::Result<String> {
    warehouse
        .iter()
        .map(|stack| stack.last())
        .collect::<Option<String>>()
        .context("One or more stack ended up empty")
}

pub fn p1(file: &str) -> anyhow::Result<String> {
    let (initial_stack_schema, rearrangements) = file.split_once("\n\n").unwrap();

    let mut warehouse = warehouse(initial_stack_schema)?;

    // apply the rearrangements
    for rearrangement in rearrangements.lines() {
        let rearrangement = Rearrangement::from_str(rearrangement)?;

        apply_rearrangement(&mut warehouse, &rearrangement, &CraneModel::CrateMover9000);
    }

    // get the final arrangement
    crates_at_the_top(&warehouse)
}

pub fn p2(file: &str) -> anyhow::Result<String> {
    let (initial_stack_schema, rearrangements) =
        file.split_once("\n\n").context("No stack numbers row")?;

    let mut warehouse: Warehouse = warehouse(initial_stack_schema)?;

    // apply the rearrangements
    for rearrangement in rearrangements.lines() {
        let rearrangement = Rearrangement::from_str(rearrangement)?;

        apply_rearrangement(&mut warehouse, &rearrangement, &CraneModel::CrateMover9001);
    }

    // format the final arrangement
    crates_at_the_top(&warehouse)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => "CMZ"; "example")]
    #[test_case(REAL => "ZWHVFWQWW"; "real")]
    fn test_p1(inp: &str) -> String {
        p1(inp).unwrap()
    }

    #[test_case(EXAMPLE => "MCD"; "example")]
    #[test_case(REAL => "HZFZCCWWV"; "real")]
    fn test_p2(inp: &str) -> String {
        p2(inp).unwrap()
    }
}
