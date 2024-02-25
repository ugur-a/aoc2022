use itertools::Itertools;

pub fn p1(file: &str) -> usize {
    // iterate over input lines
    file.lines()
        // parse each line as assignment pairs (represented by a 4-element tuple)
        .map(|line| -> (u32, u32, u32, u32) {
            line.split(&[',', '-'][..])
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .expect("A well structured input")
        })
        // retain only the tuples where the exercise condition is met
        .filter(|&(elf1_start, elf1_end, elf2_start, elf2_end)| {
            (elf1_start <= elf2_start && elf2_end <= elf1_end)
                || (elf2_start <= elf1_start && elf1_end <= elf2_end)
        })
        // count such tuples
        .count()
}

pub fn p2(file: &str) -> usize {
    // iterate over input lines
    file.lines()
        // parse each line as assignment pairs (represented by a 4-element tuple)
        .map(|line| -> (u32, u32, u32, u32) {
            line.split(&[',', '-'][..])
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .expect("A well structured input")
        })
        // retain only the tuples where the exercise condition is met
        .filter(|&(elf1_start, elf1_end, elf2_start, elf2_end)| {
            (elf1_start <= elf2_start && elf2_start <= elf1_end)
                || (elf1_start <= elf2_end && elf2_end <= elf1_end)
                || (elf2_start <= elf1_start && elf1_start <= elf2_end)
                || (elf2_start <= elf1_end && elf1_end <= elf2_end)
        })
        // count such tuples
        .count()
}
