use libaoc::points::Point2D;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Pos = Point2D<isize>;

fn parse_map(s: &str) -> HashSet<Pos> {
    let mut elf_positions = HashSet::with_capacity(s.chars().filter(|c| c == &'#').count());
    for (y, line) in s.lines().enumerate() {
        for (x, char) in line.char_indices() {
            if char == '#' {
                let pos = Point2D(x as isize, y as isize);
                elf_positions.insert(pos);
            }
        }
    }
    elf_positions
}

pub fn show_map(round: usize, positions: &[Pos]) {
    println!("== End of Round {round} ==");
    let Border2D {
        left,
        right,
        top,
        down,
    } = min_enclosing_rectangle(positions.iter(), positions.iter());

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

fn min_enclosing_rectangle<'a, I>(positions1: I, positions2: I) -> Border2D<isize>
where
    I: Iterator<Item = &'a Pos>,
{
    let (left, right) = positions1.map(Point2D::x).minmax().into_option().unwrap();
    let (top, down) = positions2.map(Point2D::y).minmax().into_option().unwrap();

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

fn first_half(
    elf_positions: &HashSet<Pos>,
    elf_dibs: &mut HashMap<Pos, Pos>,
    dibs_counts: &mut HashMap<Pos, usize>,
    directions_order: &mut [RangeInclusive<usize>; 4],
) {
    for pos in elf_positions {
        let adj_positions = DIRECTIONS.map(|dir| adj_pos(*pos, &dir));

        // don't do anything if no elves around
        // TODO: reuse the result of these reads when looking at each direction separately later
        if adj_positions.iter().all(|pos| !elf_positions.contains(pos)) {
            continue;
        }

        // look at each side, and move there if free
        for pos_triplet in directions_order
            .clone()
            .map(|direction| &adj_positions[direction])
        {
            if pos_triplet.iter().any(|pos| elf_positions.contains(pos)) {
                continue;
            }

            elf_dibs.insert(*pos, pos_triplet[1]);
            *dibs_counts.entry(pos_triplet[1]).or_insert(0) += 1;
            break;
        }
    }

    // rotate the order of the considered directions for the next round
    directions_order.rotate_left(1);
}

fn second_half(
    elf_positions: &mut HashSet<Pos>,
    elf_dibs: &mut HashMap<Pos, Pos>,
    dibs_counts: &mut HashMap<Pos, usize>,
) -> usize {
    let mut n_moves = 0;
    *elf_positions = elf_positions
        .drain()
        .map(|pos| {
            // don't do anything if haven't placed dibs in the first half
            if let Some(dibs) = elf_dibs.remove(&pos) {
                // don't actually move if others have dibs on the same space
                if dibs_counts[&dibs] == 1 {
                    n_moves += 1;
                    dibs
                } else {
                    pos
                }
            } else {
                pos
            }
        })
        .collect();

    elf_dibs.clear();
    dibs_counts.clear();
    n_moves
}

pub fn p1(file: &str) -> usize {
    const N_ROUNDS: usize = 10;

    let mut elf_positions = parse_map(file);

    let mut elf_dibs = HashMap::with_capacity(elf_positions.len());
    let mut dibs_counts = HashMap::with_capacity(elf_positions.len());
    let mut directions_order = [0..=2, 4..=6, 2..=4, 6..=8];
    for _ in 0..N_ROUNDS {
        first_half(
            &elf_positions,
            &mut elf_dibs,
            &mut dibs_counts,
            &mut directions_order,
        );

        second_half(&mut elf_positions, &mut elf_dibs, &mut dibs_counts);
    }

    // minimal spanning rectangle
    let n_ground = {
        let Border2D {
            left,
            right,
            top,
            down,
        } = min_enclosing_rectangle(elf_positions.iter(), elf_positions.iter());
        let width: usize = (right - left + 1).try_into().unwrap();
        let height: usize = (down - top + 1).try_into().unwrap();
        width * height - elf_positions.len()
    };
    n_ground
}

pub fn p2(file: &str) -> usize {
    let mut elf_positions = parse_map(file);

    let mut elf_dibs = HashMap::with_capacity(elf_positions.len());
    let mut dibs_counts = HashMap::with_capacity(elf_positions.len());
    let mut directions_order = [0..=2, 4..=6, 2..=4, 6..=8];
    for round in 1.. {
        first_half(
            &elf_positions,
            &mut elf_dibs,
            &mut dibs_counts,
            &mut directions_order,
        );

        let n_moves = second_half(&mut elf_positions, &mut elf_dibs, &mut dibs_counts);

        if n_moves == 0 {
            return round;
        }
    }
    // Rust isn't smart enough to realise that the loop _will_ run at least once and return a result
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 110)]
    #[test_case(REAL => 3987)]
    fn test_p1(inp: &str) -> usize {
        p1(inp)
    }
    #[test_case(EXAMPLE => 20)]
    #[test_case(REAL => 938)]
    fn test_p2(inp: &str) -> usize {
        p2(inp)
    }
}
