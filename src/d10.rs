use std::collections::HashMap;

pub fn p1(file: &str) -> i32 {
    let mut storage = HashMap::new();
    let mut current_cycle = 0;
    let mut x = 1;
    let interesting_cycles = (20..=220).step_by(40).collect::<Vec<_>>();
    for line in file.lines() {
        if let Some(("addx", num)) = line.split_once(' ') {
            x += num.parse::<i32>().unwrap();
            current_cycle += 2;
            storage.insert(current_cycle, x);
        // noop
        } else {
            current_cycle += 1;
        }
    }

    interesting_cycles
        .into_iter()
        .map(|cycle| {
            let mut searcher = cycle;
            loop {
                match storage.get(&(searcher - 1)) {
                    Some(value_of_x) => return value_of_x * cycle,
                    None => searcher -= 1
                }
            };
        })
        .sum()
}

pub fn p2(file: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d10/test.txt").unwrap();
        assert_eq!(p1(&inp), 13140);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d10/real.txt").unwrap();
        assert_eq!(p1(&inp), 15360);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/d10/test.txt").unwrap();
        assert_eq!(p2(&inp), 8);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/d10/real.txt").unwrap();
        assert_eq!(p2(&inp), 0);
    }
}
