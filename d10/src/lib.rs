fn parse_operations(file: &str, init_value: i32) -> Vec<i32> {
    let mut storage = Vec::new();
    let mut x = init_value;
    for line in file.lines() {
        if let Some(("addx", num)) = line.split_once(' ') {
            storage.push(x);
            storage.push(x);
            x += num.parse::<i32>().unwrap();
        // noop
        } else {
            storage.push(x);
        }
    }
    storage
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
pub fn p1(file: &str) -> i32 {
    let interesting_cycles = (20..=220).step_by(40).collect::<Vec<_>>();

    let storage = parse_operations(file, 1);

    interesting_cycles
        .into_iter()
        .map(|cycle| cycle as i32 * storage[cycle - 1])
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

    let storage = parse_operations(file, 1);

    let rows: Vec<String> = (0..crt.height)
        .map(|row_num| {
            (0..crt.width)
                .map(|col_num| {
                    let cycle = crt.width * row_num + col_num;

                    // only check against the horizontal position of the sprite
                    let crt_position = col_num;

                    let center_of_sprite = storage[cycle];

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

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/test.txt");
        assert_eq!(p1(inp), 13140);
    }
    #[test]
    fn real_p1() {
        let inp = include_str!("../inputs/real.txt");
        assert_eq!(p1(inp), 15360);
    }
    #[test]
    fn test_p2() {
        let inp = include_str!("../inputs/test.txt");
        let out = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(p2(inp), out);
    }
    #[test]
    fn real_p2() {
        let inp = include_str!("../inputs/real.txt");
        let out = "###..#..#.#....#..#...##..##..####..##..
#..#.#..#.#....#..#....#.#..#....#.#..#.
#..#.####.#....####....#.#......#..#..#.
###..#..#.#....#..#....#.#.##..#...####.
#....#..#.#....#..#.#..#.#..#.#....#..#.
#....#..#.####.#..#..##...###.####.#..#.";
        assert_eq!(p2(inp), out);
    }
}
