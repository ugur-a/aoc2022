use std::collections::HashSet;

use intersection::hash_set;
use itertools::Itertools;

fn to_priority(char: char) -> usize {
    match char {
        'a'..='z' => (char as usize) - 96,
        'A'..='Z' => (char as usize) - 64 + 26,
        _ => unreachable!(),
    }
}

pub fn p1(file: &str) -> usize {
    file.lines()
        // split into compartments
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(compartment1, compartment2)| (compartment1, compartment2).into())
        // find the common item
        .map(|compartments: [&str; 2]| {
            compartments.map(|compartment| compartment.chars().collect::<HashSet<char>>())
        })
        .map(|[compartment1, compartment2]| {
            *compartment1
                .intersection(&compartment2)
                .exactly_one()
                .unwrap()
        })
        // calculate its priority
        .map(to_priority)
        // add up the priorities
        .sum()
}

pub fn p2(file: &str) -> usize {
    file.lines()
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
        // get chunks of 3 backpacks
        .chunks(3)
        .into_iter()
        // in each chunk, find the common item (the badge)
        .map(|chunk| hash_set::intersection(chunk).drain().exactly_one().unwrap())
        // calculate its priority
        .map(to_priority)
        // add up the priorities
        .sum()
}
