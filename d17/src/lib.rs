use anyhow::bail;
use itertools::Itertools;
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
    }

    /// how many previous rows of the chamber are saved
    const HIST_SIZE: usize = 300;
    fn snapshot(&self) -> Box<[u8]> {
        self.occupied_points
            .iter()
            .rev()
            .take(Self::HIST_SIZE)
            .copied()
            .collect()
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

    let mut rocks = ROCKS.into_iter().enumerate().cycle();

    let mut pushes = {
        let pushes: Vec<_> = file.trim_end().chars().map(Jet::try_from).try_collect()?;
        pushes.into_iter().enumerate().cycle()
    };

    let mut heights_after_rounds = Vec::new();
    let mut seen_states = HashMap::new();

    for round_i in 0..num_rounds {
        let (rock_i, rock) = rocks.next().expect("`rocks` is a cycle, so won't end");
        let mut raa = chamber.new_raa(rock);

        'falling: loop {
            // jet stream
            // don't care if couldn't be pushed sideways
            let (jet_i, jet) = pushes.next().expect("`pushes` is a cycle, so won't end");
            let _ = match jet {
                Jet::Left => chamber.try_push_left(&mut raa),
                Jet::Right => chamber.try_push_right(&mut raa),
            };

            if chamber.try_fall(&mut raa).is_ok() {
                continue 'falling;
            }

            // otherwise, come to rest
            chamber.add(raa);
            heights_after_rounds.push(chamber.height());

            // check whether a similar (modulo hist_size) state was achieved before
            let k = (rock_i, jet_i, chamber.snapshot());
            match seen_states.get(&k) {
                None => {
                    seen_states.insert(k, round_i);
                    break;
                }
                Some(&prev_round_i) => {
                    // no need to run the actual simulation any further

                    // calculate the height after num_rounds by extrapolating:
                    //
                    // rounds are in one of 3 parts:
                    // - before the first cycle: every state seen so far has been unique
                    // - during cycles: starting from a state, after `n` rounds the same state is achieved (modulo hist_size)
                    //   this is repeated as long as `n` fits in remaining rounds
                    // - after last cycle: the remaining `m` rounds that happen after the last full cycle is made
                    //   this repeats the `m` first rounds of a cycle
                    //
                    // with this, the total height after `num_rounds` can be calculated by adding up:
                    // - height gathered before the first cycle
                    // - height/cycle * #cycles
                    // - height gathered after the last cycle
                    //   since this part repeats a regular cycle up to `m` rounds,
                    //   the gathered height can be looked up in `heights_after_rounds`
                    let rounds_at_start_1st_cycle = prev_round_i;
                    let rounds_at_start_2nd_cycle = round_i;
                    let d_rounds_per_cycle = rounds_at_start_2nd_cycle - rounds_at_start_1st_cycle;

                    let n_cycles = (num_rounds - rounds_at_start_1st_cycle) / d_rounds_per_cycle;

                    let d_rounds_after_cycles =
                        (num_rounds - rounds_at_start_1st_cycle) % d_rounds_per_cycle;

                    let h_at_start_1st_cycle = heights_after_rounds[rounds_at_start_1st_cycle];
                    let h_at_start_2nd_cycle = heights_after_rounds[rounds_at_start_2nd_cycle];
                    let d_h_per_cycle = h_at_start_2nd_cycle - h_at_start_1st_cycle;

                    let d_h_after_cycles = heights_after_rounds
                        [rounds_at_start_1st_cycle + d_rounds_after_cycles - 1]
                        - h_at_start_1st_cycle;

                    let h_after_cycles =
                        h_at_start_1st_cycle + d_h_per_cycle * n_cycles + d_h_after_cycles;

                    return Ok(h_after_cycles);
                }
            }
        }
    }
    // if hasn't encountered any cycles during the entire simulation (can't happen in d17)
    // just return the final height
    Ok(chamber.height())
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
    use PushRockErr::OutOfBounds as Oob;

    #[test]
    fn try_push_sideways() {
        let chamber = Chamber::new();
        let rock = ROCKS[0];
        let mut raa = chamber.new_raa(rock);

        assert_eq!(chamber.try_push_right(&mut raa), Ok(()));
        assert_eq!(raa.rock.inner, [15, 0, 0, 0]);
        assert_eq!(chamber.try_push_right(&mut raa), Err(Oob));
        assert_eq!(raa.rock.inner, [15, 0, 0, 0]);
        assert_eq!(chamber.try_push_left(&mut raa), Ok(()));
        assert_eq!(raa.rock.inner, [30, 0, 0, 0]);
        assert_eq!(chamber.try_push_left(&mut raa), Ok(()));
        assert_eq!(raa.rock.inner, [60, 0, 0, 0]);
        assert_eq!(chamber.try_push_left(&mut raa), Ok(()));
        assert_eq!(raa.rock.inner, [120, 0, 0, 0]);
        assert_eq!(chamber.try_push_left(&mut raa), Err(Oob));
        assert_eq!(raa.rock.inner, [120, 0, 0, 0]);
    }
    #[test]
    fn try_fall() {
        let chamber = Chamber::new();
        let rock = ROCKS[0];
        let mut raa = chamber.new_raa(rock);

        assert_eq!(chamber.try_fall(&mut raa), Ok(()));
        assert_eq!(raa.altitude, 2);
        assert_eq!(chamber.try_fall(&mut raa), Ok(()));
        assert_eq!(raa.altitude, 1);
        assert_eq!(chamber.try_fall(&mut raa), Ok(()));
        assert_eq!(raa.altitude, 0);
        assert_eq!(chamber.try_fall(&mut raa), Err(Oob));
        assert_eq!(raa.altitude, 0);
    }
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
    fn real_p2() {
        let inp = read_to_string("inputs/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 1_602_881_844_347);
    }
}
