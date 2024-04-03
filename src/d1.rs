use anyhow::Context;
use itertools::Itertools;

pub fn p1(file: &str) -> anyhow::Result<u32> {
    // split elves inventories
    file.split("\n\n")
        // calculate each elf's total calories
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .context("No goblins")
}

pub fn p2(file: &str) -> u32 {
    // split elves inventories
    file.split("\n\n")
        // calculate each elf's total calories
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_unstable()
        .rev()
        .take(3)
        .sum()
}
