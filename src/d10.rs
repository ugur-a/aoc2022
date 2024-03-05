use itertools::Itertools;
use nohash_hasher::IntMap;

pub fn p1(file: &str) -> i32 {
    let mut storage = IntMap::default();
    let mut current_cycle = 0_u32;
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
                    Some(value_of_x) => return value_of_x * cycle as i32,
                    None => searcher -= 1,
                }
            }
        })
        .sum()
}

pub fn p2(file: &str) -> String {
    let mut storage = IntMap::default();

    let mut current_cycle = 0;
    let mut x = 1;
    storage.insert(current_cycle, x);
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

    struct Crt {
        width: u32,
        height: u32,
    }

    let crt = Crt {
        width: 40,
        height: 6,
    };

    (0..crt.height)
        .map(|row_num| {
            (0..crt.width)
                .map(|col_num| {
                    let cycle = crt.width * row_num + col_num;
                    // only check against the horizontal position of the sprite
                    let crt_position = col_num;

                    let mut searcher = cycle;
                    let center_of_sprite = loop {
                        match storage.get(&searcher) {
                            Some(value) => break value,
                            None => searcher -= 1,
                        }
                    };

                    if center_of_sprite.abs_diff(crt_position as i32) <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n")
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
    fn test_p2() {
        let inp = read_to_string("inputs/d10/test.txt").unwrap();
        let out = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(p2(&inp), out);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/d10/real.txt").unwrap();
        let out = "###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.";
        assert_eq!(p2(&inp), out);
    }
}
