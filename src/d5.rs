use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::{Error, Result};
use itertools::{repeat_n, Itertools};

struct Rearrangement {
    num_crates_to_move: usize,
    stack_to_move_to: usize,
    stack_to_take_from: usize,
}

struct Warehouse {
    stacks: Vec<Vec<char>>,
}

impl Deref for Warehouse {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.stacks
    }
}

impl DerefMut for Warehouse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stacks
    }
}

impl FromStr for Warehouse {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        // remove the last row of the stack arrangement schema - the one with stack numbers
        let (initial_stack_arrangement, last_row_of_stack_arrangement) =
            s.rsplit_once('\n').unwrap();

        // since we don't need the last row anyway, use it to indirectly calculate the number of stacks
        let num_stacks = (last_row_of_stack_arrangement.len() + 1) / 4;

        // initialize the warehouse (collection of stacks)
        let mut stacks: Vec<Vec<char>> = (0..num_stacks).map(|_| Vec::new()).collect_vec();

        // parse the initial stack arrangement - fill up the warehouse
        // comment: go over lines bottom-up, since that's how the crates are stacked
        for line in initial_stack_arrangement.lines().rev() {
            line.chars()
                .chunks(4)
                .into_iter()
                // provide the stack number for each maybe-crate
                .enumerate()
                // if there's a crate, add it to the corresponding stack, skip if only air
                .filter_map(|(idx, chunk)| match chunk.collect_vec().as_slice() {
                    ['[', crate_name, ']', ..] => Some((idx, *crate_name)),
                    [' ', ' ', ' ', ..] => None,
                    _ => unreachable!(),
                })
                .for_each(|(idx, crate_name)| {
                    stacks.get_mut(idx).unwrap().push(crate_name);
                });
        }
        Ok(Self { stacks })
    }
}

pub fn p1(file: &str) -> Result<String> {
    let (initial_stack_schema, rearrangements) = file.split_once("\n\n").unwrap();

    let mut warehouse = Warehouse::from_str(initial_stack_schema)?;

    // apply the rearrangements
    for rearrangement in rearrangements.lines() {
        let (num_crates_to_move, stack_to_move_to, stack_to_take_from) =
            match rearrangement.split_whitespace().collect_vec().as_slice() {
                [_, num_crates_to_move, _, stack_to_move_from, _, stack_to_move_to, ..] => (
                    num_crates_to_move.parse::<usize>()?,
                    stack_to_move_to.parse::<usize>()? - 1,
                    stack_to_move_from.parse::<usize>()? - 1,
                ),
                _ => unreachable!(),
            };

        let current_length_of_stack_to_move_from = warehouse.get(stack_to_take_from).unwrap().len();

        let crates_to_move = warehouse
            .get_mut(stack_to_take_from)
            .unwrap()
            .drain((current_length_of_stack_to_move_from - num_crates_to_move)..)
            .rev()
            .collect_vec();

        warehouse
            .get_mut(stack_to_move_to)
            .unwrap()
            .extend(crates_to_move);
    }

    // format the final arrangement
    Ok(warehouse
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect())
}

pub fn p2(file: &str) -> Result<String> {
    let (initial_stack_schema, rearrangements) = file.split_once("\n\n").unwrap();

    let mut warehouse = Warehouse::from_str(initial_stack_schema)?;

    // apply the rearrangements
    for rearrangement in rearrangements.lines() {
        let (num_crates_to_move, stack_to_move_to, stack_to_take_from) =
            match rearrangement.split_whitespace().collect_vec().as_slice() {
                [_, num_crates_to_move, _, stack_to_move_from, _, stack_to_move_to, ..] => (
                    num_crates_to_move.parse::<usize>()?,
                    stack_to_move_to.parse::<usize>()? - 1,
                    stack_to_move_from.parse::<usize>()? - 1,
                ),
                _ => unreachable!(),
            };

        let current_length_of_stack_to_move_from = warehouse.get(stack_to_take_from).unwrap().len();

        let crates_to_move = warehouse
            .get_mut(stack_to_take_from)
            .unwrap()
            .drain((current_length_of_stack_to_move_from - num_crates_to_move)..)
            // the only difference from p1 - don't reverse the crates when moving
            .collect_vec();

        warehouse
            .get_mut(stack_to_move_to)
            .unwrap()
            .extend(crates_to_move);
    }

    // format the final arrangement
    Ok(warehouse
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect())
}
