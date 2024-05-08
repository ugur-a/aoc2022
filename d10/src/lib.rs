fn operations(file: &str, init_value: i32) -> BTreeMap<usize, i32> {
    let mut register_history = BTreeMap::new();
    let mut cycle = 0;
    let mut register_value = init_value;
    for line in file.lines() {
        if let Some(("addx", num)) = line.split_once(' ') {
            cycle += 2;
            register_value += num.parse::<i32>().unwrap();
            register_history.insert(cycle, register_value);
        // noop
        } else {
            cycle += 1;
        }
    }
    register_history
}

trait BiggestPrevious<Q> {
    type Item;
    fn biggest_previous(&self, query: Q) -> Option<&Self::Item>;
}

impl BiggestPrevious<usize> for BTreeMap<usize, i32> {
    type Item = i32;
    fn biggest_previous(&self, query: usize) -> Option<&Self::Item> {
        self.range(..=query).next_back().map(|(_u, i)| i)
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn p1(file: &str) -> i32 {
    let interesting_cycles = (20..=220).step_by(40).collect::<Vec<_>>();

    let register_history = operations(file, 1);

    interesting_cycles
        .into_iter()
        .map(|cycle| cycle as i32 * register_history.biggest_previous(cycle - 1).unwrap())
        .sum()
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn p2(file: &str) -> String {
    struct Crt {
        width: usize,
        height: usize,
    }

    let crt = Crt {
        width: 40,
        height: 6,
    };

    let register_history = operations(file, 1);

    let rows: Vec<String> = (0..crt.height)
        .map(|row_num| {
            (0..crt.width)
                .map(|col_num| {
                    let cycle = crt.width * row_num + col_num;

                    // only check against the horizontal position of the sprite
                    let crt_position = col_num;

                    let center_of_sprite = register_history.biggest_previous(cycle).unwrap();

                    if center_of_sprite.abs_diff(crt_position as i32) <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect();
    rows.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp), 13140);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp), 15360);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
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
        let inp = read_to_string("inputs/real.txt").unwrap();
        let out = "###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.";
        assert_eq!(p2(&inp), out);
    }
}
