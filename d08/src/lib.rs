use anyhow::{anyhow, Context};
use itertools::Itertools;

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

fn forest(file: &str) -> anyhow::Result<Vec<Vec<Tree>>> {
    file.lines()
        .map(|line| {
            line.chars()
                .map(|char| match char.to_digit(10) {
                    Some(n) => Ok(n + 1),
                    None => Err(anyhow!("invalid height: {char}")),
                })
                .map_ok(Tree::with_height)
                .try_collect()
        })
        .try_collect()
}

/// Less exact than [`check_scenicities_in_a_line`] - checks whether
/// each tree is visible, i.e. has a scenicity value of 0
fn check_visibilities_in_a_line(line: &mut [Tree]) {
    // save the highest tree of the line so that we
    // don't check past it coming from both directions
    let position_highest_tree = line.iter().position_max_by_key(|tree| tree.height).unwrap();

    // check the line forwards until the highest tree
    let mut current_max_height = u32::MIN;
    for tree in &mut line[..position_highest_tree] {
        if tree.height <= current_max_height {
            continue;
        }
        current_max_height = tree.height;
        tree.mark_visible();
    }

    let mut current_max_height = u32::MIN;
    for tree in line[position_highest_tree..].iter_mut().rev() {
        if tree.height <= current_max_height {
            continue;
        }
        current_max_height = tree.height;
        tree.mark_visible();
    }
}

pub fn p1(file: &str) -> anyhow::Result<usize> {
    // create the map
    let mut forest: Vec<Vec<Tree>> = forest(file)?;

    // analyze visibility horizontally
    for row in &mut forest {
        check_visibilities_in_a_line(row);
    }

    // transpose the map so that iterating vertically isn't so cache-miss-prone
    forest = forest.transpose_out_of_place();

    // analyze visibility vertically
    for col in &mut forest {
        check_visibilities_in_a_line(col);
    }

    Ok(forest
        .into_iter()
        .flatten()
        .filter(Tree::is_visible)
        .count())
}

/// More exact than [`check_visibilities_in_a_line`] - gets the exact scenicity values
fn check_scenicities_in_a_line(line: &mut [Tree]) {
    let scenicities = line
        .iter()
        .enumerate()
        .map(|(tree_position, tree)| {
            if tree.is_visible() {
                return 0;
            }

            let current_scenicity_forward = line[(tree_position + 1)..]
                .iter()
                .take_while_inclusive(|other_tree| tree.height > other_tree.height)
                .count();

            let current_scenicity_backwards = line[..tree_position]
                .iter()
                .rev()
                .take_while_inclusive(|other_tree| tree.height > other_tree.height)
                .count();

            current_scenicity_backwards * current_scenicity_forward
        })
        .collect_vec();

    std::iter::zip(line, scenicities).for_each(|(tree, scenicity)| tree.scenicity *= scenicity);
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let mut forest: Vec<Vec<Tree>> = forest(file)?;

    for row in &mut forest {
        check_scenicities_in_a_line(row);
    }

    forest = forest.transpose_out_of_place();

    for col in &mut forest {
        check_scenicities_in_a_line(col);
    }

    forest
        .into_iter()
        .flatten()
        .map(|tree| tree.scenicity)
        .max()
        .context("empty forest")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 21)]
    #[test_case(REAL => 1708)]
    fn test_p1(inp: &str) -> usize {
        p1(inp).unwrap()
    }
    #[test_case(EXAMPLE => 8)]
    #[test_case(REAL => 504_000)]
    fn test_p2(inp: &str) -> usize {
        p2(inp).unwrap()
    }
}
