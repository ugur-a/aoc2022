use anyhow::{Context, Error, Result};
use itertools::Itertools;

trait IntoForest {
    type Err;

    fn to_forest(self) -> Result<Vec<Vec<Tree>>, Self::Err>
    where
        Self: Sized;
}

impl IntoForest for &str {
    type Err = Error;

    fn to_forest(self) -> Result<Vec<Vec<Tree>>, Self::Err> {
        Ok(self
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() + 1)
                    .map(Tree::with_height)
                    .collect()
            })
            .collect())
    }
}

/// Returns the transposed copy of a collection
trait TransposeOutOfPlace {
    fn transpose_out_of_place(self) -> Self;
}

impl<T> TransposeOutOfPlace for Vec<Vec<T>> {
    /// stolen from: <https://stackoverflow.com/questions/39775060/reverse-iterating-over-a-vec-versus-vec-iter>
    fn transpose_out_of_place(self) -> Self {
        assert!(!self.is_empty());
        let len = self[0].len();
        let mut iters = self
            .into_iter()
            .map(IntoIterator::into_iter)
            .collect::<Vec<_>>();
        (0..len)
            .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
            .collect()
    }
}

struct Tree {
    height: u32,
    scenicity: usize,
}
impl Tree {
    fn with_height(height: u32) -> Self {
        Self {
            height,
            scenicity: 1,
        }
    }

    fn is_visible(&self) -> bool {
        self.scenicity == 0
    }
    fn mark_visible(&mut self) {
        self.scenicity = 0;
    }
}

trait Visibility {
    fn check_visibilities(&mut self);
}

impl Visibility for &mut Vec<Tree> {
    /// Less exact than [`check_scenicities_in_a_line`] - checks whether
    /// each tree is visible, i.e. has a scenicity value of 0
    fn check_visibilities(&mut self) {
        // save the highest tree of the line so that we
        // don't check past it coming from both directions
        let position_highest_tree = self.iter().position_max_by_key(|tree| tree.height).unwrap();

        // check the line forwards until the highest tree
        let mut current_max_height = u32::MIN;
        for tree in &mut self[..position_highest_tree] {
            if tree.height <= current_max_height {
                continue;
            }
            current_max_height = tree.height;
            tree.mark_visible();
        }

        let mut current_max_height = u32::MIN;
        for tree in self[position_highest_tree..].iter_mut().rev() {
            if tree.height <= current_max_height {
                continue;
            }
            current_max_height = tree.height;
            tree.mark_visible();
        }
    }
}

pub fn p1(file: &str) -> Result<usize> {
    // create the map
    let mut forest = file.to_forest()?;

    // analyze visibility horizontally
    for mut row in &mut forest {
        row.check_visibilities();
    }

    // transpose the map so that iterating vertically isn't so cache-miss-prone
    forest = forest.transpose_out_of_place();

    // analyze visibility vertically
    for mut col in &mut forest {
        col.check_visibilities();
    }

    Ok(forest
        .iter()
        .flatten()
        .filter(|tree| tree.is_visible())
        .count())
}

trait Scenicity {
    fn check_scenicities(&mut self);
}

impl Scenicity for Vec<Tree> {
    /// More exact than [`check_visibilities_in_a_line`] - gets the exact scenicity values
    fn check_scenicities(&mut self) {
        let scenicities = self
            .iter()
            .enumerate()
            .map(|(tree_position, tree)| {
                if tree.is_visible() {
                    return 0;
                }

                let current_scenicity_forward = self[(tree_position + 1)..]
                    .iter()
                    .take_while_inclusive(|other_tree| tree.height > other_tree.height)
                    .count();

                let current_scenicity_backwards = self[..tree_position]
                    .iter()
                    .rev()
                    .take_while_inclusive(|other_tree| tree.height > other_tree.height)
                    .count();

                current_scenicity_backwards * current_scenicity_forward
            })
            .collect_vec();

        self.iter_mut()
            .zip(scenicities.iter())
            .for_each(|(tree, scenicity)| tree.scenicity *= scenicity);
    }
}

pub fn p2(file: &str) -> Result<usize> {
    let mut forest = file.to_forest()?;

    for row in &mut forest {
        row.check_scenicities();
    }

    forest = forest.transpose_out_of_place();

    for col in &mut forest {
        col.check_scenicities();
    }

    forest
        .iter()
        .flatten()
        .map(|tree| tree.scenicity)
        .max()
        .context("No trees to get the heighest")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_p1() {
        let inp = read_to_string("inputs/d8/test.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 21);
    }
    #[test]
    fn real_p1() {
        let inp = read_to_string("inputs/d8/real.txt").unwrap();
        assert_eq!(p1(&inp).unwrap(), 1708);
    }
    #[test]
    fn test_p2() {
        let inp = read_to_string("inputs/d8/test.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 8);
    }
    #[test]
    fn real_p2() {
        let inp = read_to_string("inputs/d8/real.txt").unwrap();
        assert_eq!(p2(&inp).unwrap(), 504_000);
    }
}
