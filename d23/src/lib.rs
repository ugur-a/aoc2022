use aoc2022lib::points::Point2D;
use itertools::Itertools;
use std::collections::HashMap;

type Pos = Point2D<usize>;

fn parse_map(s: &str, buf_width: usize) -> Vec<Pos> {
    let mut elf_positions = Vec::new();
    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.char_indices() {
            if char == '#' {
                let pos = Point2D(x + buf_width, y + buf_width);
                elf_positions.push(pos);
            }
        }
    }
    elf_positions
}

#[allow(dead_code)]
fn show_map(round: usize, positions: &[Pos]) {
    println!("== End of Round {round} ==");
    let Border2D {
        left,
        right,
        top,
        down,
    } = min_enclosing_rectangle(positions);

    for y in top..=down {
        for x in left..=right {
            let pos = Point2D(x, y);
            if positions.iter().any(|p| p == &pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!(" {y}");
    }
}

#[derive(Debug)]
struct Border2D<T, U = T> {
    left: T,
    right: T,
    top: U,
    down: U,
}

fn min_enclosing_rectangle(positions: &[Pos]) -> Border2D<usize> {
    let (left, right) = positions
        .iter()
        .map(aoc2022lib::points::Point2D::x)
        .minmax()
        .into_option()
        .unwrap();
    let (top, down) = positions
        .iter()
        .map(aoc2022lib::points::Point2D::y)
        .minmax()
        .into_option()
        .unwrap();

    Border2D {
        left,
        right,
        top,
        down,
    }
}

enum Direction {
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
}

const DIRECTIONS: [Direction; 9] = {
    use Direction as D;
    [D::NE, D::N, D::NW, D::W, D::SW, D::S, D::SE, D::E, D::NE]
};

fn adj_pos(Point2D(x, y): Pos, dir: &Direction) -> Pos {
    use Direction as D;
    let (new_col, new_row) = match dir {
        D::NE => (x + 1, y - 1),
        D::N => (x, y - 1),
        D::NW => (x - 1, y - 1),
        D::W => (x - 1, y),
        D::SW => (x - 1, y + 1),
        D::S => (x, y + 1),
        D::SE => (x + 1, y + 1),
        D::E => (x + 1, y),
    };
    Point2D(new_col, new_row)
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    const N_ROUNDS: usize = 10;
    // the further an elf can end up from the starting square
    // - in case it starts at the border and goes away from the center each time
    const BUF_WIDTH: usize = N_ROUNDS;

    let mut elf_positions = parse_map(file, BUF_WIDTH);

    let mut elf_dibs = HashMap::with_capacity(elf_positions.len());
    let mut dibs_counts = HashMap::with_capacity(elf_positions.len());
    let mut directions_order = [0..=2, 4..=6, 2..=4, 6..=8];
    for _ in 0..N_ROUNDS {
        // first half
        for pos in &elf_positions {
            let adj_positions = DIRECTIONS.map(|dir| adj_pos(*pos, &dir));

            // don't do anything if no elves around
            if adj_positions
                .iter()
                .all(|pos| !elf_positions.iter().contains(pos))
            {
                continue;
            }

            // look at each side, and move there if free
            for pos_triplet in &directions_order
                .clone()
                .map(|direction| &adj_positions[direction])
            {
                if pos_triplet
                    .iter()
                    .any(|pos| elf_positions.iter().contains(pos))
                {
                    continue;
                }

                elf_dibs.insert(*pos, pos_triplet[1]);
                *dibs_counts.entry(pos_triplet[1]).or_insert(0) += 1;
                break;
            }
        }

        // second half
        for pos in &mut elf_positions {
            // don't do anything if haven't placed dibs in the first half
            let Some(dibs) = elf_dibs.remove(pos) else {
                continue;
            };

            // don't actually move if others have dibs on the same space
            if dibs_counts[&dibs] > 1 {
                continue;
            }

            *pos = dibs;
        }

        elf_dibs.clear();
        dibs_counts.clear();

        // rotate the order of the considered directions for the next round
        directions_order.rotate_left(1);
    }

    // minimal spanning rectangle
    let Border2D {
        left,
        right,
        top,
        down,
    } = min_enclosing_rectangle(&elf_positions);
    let n_ground = (right - left + 1) * (down - top + 1) - elf_positions.len();
    Ok(n_ground)
}

pub fn p2(_file: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 110);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3987);
    }
    #[test]
    #[ignore]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }
}
