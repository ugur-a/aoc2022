use anyhow::bail;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy)]
struct Rock {
    inner: [u8; 4],
    height: u8,
}

const ROCKS: [Rock; 5] = {
    // this already takes into account rocks being spawned with an offset of 2 from the left wall
    // bottom row first
    const MINUS: Rock = Rock {
        inner: [0b0001_1110, 0, 0, 0],
        height: 1,
    };
    const PLUS: Rock = Rock {
        inner: [0b0000_1000, 0b0001_1100, 0b0000_1000, 0],
        height: 3,
    };
    const RIGHT_L: Rock = Rock {
        inner: [0b0001_1100, 0b0000_0100, 0b0000_0100, 0],
        height: 3,
    };
    const I: Rock = Rock {
        inner: [0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000],
        height: 4,
    };
    const SQUARE: Rock = Rock {
        inner: [0b0001_1000, 0b0001_1000, 0, 0],
        height: 2,
    };
    [MINUS, PLUS, RIGHT_L, I, SQUARE]
};

struct RockAtAltitude {
    rock: Rock,
    altitude: usize,
}

impl Display for RockAtAltitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self { rock, altitude } = self;
        for row in (rock.inner)
            .iter()
            .copied()
            .rev()
            .chain(std::iter::repeat(0u8).take(altitude))
        {
            writeln!(f, "{row:07b}")?;
        }
        Ok(())
    }
}

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

// TODO: store already seen states (n highest rows + curr rock + curr jetstream )
// into a HashSet, and terminate after having found a state already seen
// after that, see how many rows were added during the cycle, and calculate
// the total num of rows after `num_rounds` rounds based on that
/// possible reasons for why rock couldn't be pushed
/// when using `try_push_left`/`try_push_right`/`try_fall`
#[derive(Debug, PartialEq, Eq)]
enum PushRockErr {
    /// the rock would be out of chamber bounds
    OutOfBounds,
    /// the rock would overlap with an already existing rock
    Conflict,
}

#[derive(Default)]
struct Chamber {
    occupied_points: Vec<u8>,
}

impl Chamber {
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn height(&self) -> usize {
        self.occupied_points.len()
    }

    fn new_raa(&self, rock: Rock) -> RockAtAltitude {
        RockAtAltitude {
            rock,
            altitude: self.height() + 3,
        }
    }

    fn contains(&self, &RockAtAltitude { rock, altitude }: &RockAtAltitude) -> bool {
        if self.height() < altitude {
            return false;
        }
        let occupied_rows = &self.occupied_points[altitude..];
        let rock_rows = &rock.inner;
        occupied_rows.iter().zip(rock_rows).any(|(o, r)| o & r != 0)
    }

    fn try_push_left(
        &self,
        raa @ &mut RockAtAltitude { rock, altitude }: &mut RockAtAltitude,
    ) -> Result<(), PushRockErr> {
        if rock.inner.iter().any(|row| (row & (1 << (7 - 1))) != 0) {
            return Err(PushRockErr::OutOfBounds); // already at left-most column
        }

        let pushed = RockAtAltitude {
            rock: Rock {
                inner: rock.inner.map(|row| row << 1),
                ..rock
            },
            altitude,
        };
        if self.contains(&pushed) {
            return Err(PushRockErr::Conflict);
        }
        *raa = pushed;
        Ok(())
    }

    fn try_push_right(
        &self,
        raa @ &mut RockAtAltitude { rock, altitude }: &mut RockAtAltitude,
    ) -> Result<(), PushRockErr> {
        if rock.inner.iter().any(|row| (row & 1) != 0) {
            return Err(PushRockErr::OutOfBounds); // already at right-most column
        }

        let pushed = RockAtAltitude {
            rock: Rock {
                inner: rock.inner.map(|row| row >> 1),
                ..rock
            },
            altitude,
        };
        if self.contains(&pushed) {
            return Err(PushRockErr::Conflict);
        }
        *raa = pushed;
        Ok(())
    }

    fn try_fall(
        &self,
        raa @ &mut RockAtAltitude { rock, altitude }: &mut RockAtAltitude,
    ) -> Result<(), PushRockErr> {
        if altitude == 0 {
            return Err(PushRockErr::OutOfBounds); // already on the bottom
        }

        let fallen = RockAtAltitude {
            rock,
            altitude: altitude - 1,
        };
        if self.contains(&fallen) {
            return Err(PushRockErr::Conflict);
        }
        *raa = fallen;
        Ok(())
    }

    fn trim(&mut self) {
        let _ = self
            .occupied_points
            .split_off(self.height() - Self::HEIGHT_TO_TRIM_TO);
    }

    const MAX_HEIGHT_BEFORE_TRIMMING: usize = 1024 * 1024 * 1024;
    const HEIGHT_TO_TRIM_TO: usize = 512;
    fn add(&mut self, RockAtAltitude { rock, altitude }: RockAtAltitude) {
        let rh = rock.height as usize;
        for h in 0..rh {
            if let Some(row) = self.occupied_points.get_mut(altitude + h) {
                *row |= rock.inner[h];
            } else {
                self.occupied_points.extend(&rock.inner[h..rh]);
                break;
            }
        }

        if self.height() > Self::MAX_HEIGHT_BEFORE_TRIMMING {
            self.trim();
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height()).rev().take(20) {
            writeln!(f, "{:07b}", self.occupied_points[y])?;
        }

        Ok(())
    }
}

fn tetris(file: &str, num_rounds: usize) -> anyhow::Result<usize> {
    let mut chamber = Chamber::new();

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
        let mut raa = chamber.new_raa(rock);

        loop {
            // eprintln!("{chamber}\n");
            // jet stream
            // don't care if couldn't be pushed sideways
            let _ = match pushes.next().expect("`pushes` is a cycle, so won't end") {
                Jet::Left => chamber.try_push_left(&mut raa),
                Jet::Right => chamber.try_push_right(&mut raa),
            };

            if chamber.try_fall(&mut raa).is_ok() {
                continue;
            }

            // otherwise, come to rest
            chamber.add(raa);
            break;
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
