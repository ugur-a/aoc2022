use anyhow::bail;
use aoc2022lib::points::Point2D;
use std::{collections::HashSet, fmt::Display};

type Pos = Point2D<u8, usize>;

#[derive(Clone, Copy)]
struct Rock {
    points: [Pos; 5],
    width: u8,
    height: u8,
}

macro_rules! rock {
    [$( ( $p1:expr, $p2:expr ) ),+] => {[$( Point2D($p1, $p2) ),+]};
}

const ROCKS: [Rock; 5] = {
    const MINUS: Rock = Rock {
        points: rock![(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)],
        width: 4,
        height: 1,
    };
    const PLUS: Rock = Rock {
        points: rock![(1, 0), (0, 1), (2, 1), (1, 2), (1, 1)],
        width: 3,
        height: 3,
    };
    const RIGHT_L: Rock = Rock {
        points: rock![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        width: 3,
        height: 3,
    };
    const I: Rock = Rock {
        points: rock![(0, 0), (0, 1), (0, 2), (0, 3), (0, 0)],
        width: 1,
        height: 4,
    };
    const SQUARE: Rock = Rock {
        points: rock![(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)],
        width: 2,
        height: 2,
    };
    [MINUS, PLUS, RIGHT_L, I, SQUARE]
};

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl TryFrom<char> for Jet {
    type Error = anyhow::Error;

    fn try_from(value: char) -> anyhow::Result<Self> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            chr => bail!("Invalid char: '{}'", chr),
        }
    }
}

// TODO: store each row as an bitmask
// Since we've got 7 cols, each row is u8, which can in turn be mapped to an ASCII char
// TODO: store already seen states (n highest rows + curr rock + curr jetstream )
// into a HashSet, and terminate after having found a state already seen
// after that, see how many rows were added during the cycle, and calculate
// the total num of rows after `num_rounds` rounds based on that
#[derive(Default)]
struct Chamber {
    width: u8,
    height: usize,
    occupied_points: HashSet<Pos>,
}

impl Chamber {
    fn new(width: u8) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    fn contains(&self, q: &Pos) -> bool {
        self.occupied_points.contains(q)
    }

    fn trim_to(&mut self, height_to_trim_to: usize) {
        self.occupied_points
            .retain(|point| point.1 > self.height - height_to_trim_to);
    }

    const MAX_HEIGHT_BEFORE_TRIMMING: usize = 1024 * 1024 * 1024;
    const HEIGHT_TO_TRIM_TO: usize = 512;
    #[allow(clippy::cast_lossless)]
    fn add_rock(&mut self, rock: Rock, rock_position_relative: Pos) {
        self.occupied_points
            .extend(&rock.points.map(|point| point + rock_position_relative));
        self.height = std::cmp::max(self.height, rock_position_relative.1 + rock.height as usize);
        if self.height > Self::MAX_HEIGHT_BEFORE_TRIMMING {
            self.trim_to(Self::HEIGHT_TO_TRIM_TO);
        }
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Display for Chamber {
    #[allow(clippy::cast_possible_truncation)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = std::cmp::min(20, self.height);

        let mut res = String::with_capacity(height * self.width as usize);
        for y in (0..=height).rev() {
            for x in 0..=self.width {
                let point = Point2D(x, y);
                let repr = if self.occupied_points.contains(&point) {
                    '#'
                } else {
                    '.'
                };
                res.push(repr);
            }
            res.push('\n');
        }

        write!(f, "{res}")
    }
}

fn tetris(file: &str, num_rounds: usize) -> anyhow::Result<usize> {
    let mut chamber = Chamber::new(7);

    let rocks = ROCKS.into_iter().cycle().take(num_rounds);

    let mut pushes = {
        let mut pushes = Vec::with_capacity(file.len());
        for c in file.trim_end().chars() {
            let j = Jet::try_from(c)?;
            pushes.push(j);
        }
        pushes.into_iter().cycle()
    };

    for rock in rocks {
        let spawn_height = chamber.height() + 3;
        let mut rock_position_relative = Point2D(2, spawn_height);
        loop {
            // eprintln!("{chamber}\n");
            // jet stream
            match pushes.next().unwrap() {
                Jet::Left => {
                    if rock_position_relative.0 > 0
                        && rock
                            .points
                            .iter()
                            .map(|&point| point + rock_position_relative)
                            .map(|Point2D(x, y)| Point2D(x - 1, y))
                            .all(|point| !chamber.contains(&point))
                    {
                        rock_position_relative.0 -= 1;
                    }
                }
                Jet::Right => {
                    if rock_position_relative.0 + rock.width < chamber.width
                        && rock
                            .points
                            .iter()
                            .map(|&point| point + rock_position_relative)
                            .map(|Point2D(x, y)| Point2D(x + 1, y))
                            .all(|point| !chamber.contains(&point))
                    {
                        rock_position_relative.0 += 1;
                    }
                }
            }
            // come to rest if:
            // 1) arrived at the lowest point
            if rock_position_relative.1 == 0 {
                chamber.add_rock(rock, rock_position_relative);
                break;
            }
            // 2) there's a rock point directly underneath
            let rock_stops = rock
                .points
                .iter()
                .map(|&point| point + rock_position_relative)
                .map(|Point2D(x, y)| Point2D(x, y - 1))
                .any(|point| chamber.contains(&point));
            if rock_stops {
                chamber.add_rock(rock, rock_position_relative);
                break;
                //fall
            }
            rock_position_relative.1 -= 1;
        }
    }
    Ok(chamber.height())
}

pub fn p_mid(file: &str) -> anyhow::Result<usize> {
    tetris(file, 1_000_000)
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    tetris(file, 2022)
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    tetris(file, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3068);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 3206);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 1_514_285_714_288);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 0);
    }

    #[test]
    fn test_p_mid() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p_mid(&inp).unwrap(), 1_602_842);
    }
}
