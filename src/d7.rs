use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use itertools::Itertools;

type FilesWithSizes = HashMap<PathBuf, u32>;

fn parse_files_with_sizes(s: &str) -> Result<FilesWithSizes> {
    let mut current_path = PathBuf::new();
    let mut files_with_sizes: FilesWithSizes = HashMap::new();

    for input_and_output in s.split("\n$ ") {
        if let Some(("cd", dir_name)) = input_and_output.split_once(' ') {
            current_path = match dir_name {
                ".." => current_path
                    .parent()
                    .map_or_else(PathBuf::new, std::path::Path::to_path_buf),
                _ => current_path.join(dir_name),
            }
        } else if let Some(("ls", dir_contents)) = input_and_output.split_once('\n') {
            let new_files = dir_contents
                .lines()
                .map(|line| line.split_once(' ').context("Invalid `ls` output"))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .filter_map(|(first, second)| match (first, second) {
                    ("dir", _dir_name) => None,
                    (file_size, file_name) => {
                        let file_path = current_path.join(file_name);
                        let file_size = file_size.parse().unwrap();
                        Some((file_path, file_size))
                    }
                });
            files_with_sizes.extend(new_files);
        }
    }
    Ok(files_with_sizes)
}

type DirsWithSizes = HashMap<PathBuf, u32>;

fn get_dir_sizes(files_with_sizes: &FilesWithSizes) -> DirsWithSizes {
    files_with_sizes
        .iter()
        .flat_map(|(file_path, file_size)| {
            file_path
                .ancestors()
                .skip(1)
                .map(|ancestor_path| (ancestor_path.to_path_buf(), *file_size))
        })
        .into_grouping_map()
        .sum()
}

pub fn p1(file: &str) -> Result<u32> {
    let upper_bound = 100_000u32;

    let navigations = &file[2..];
    let files_with_sizes = parse_files_with_sizes(navigations)?;

    let dirs_with_sizes = get_dir_sizes(&files_with_sizes);

    Ok(dirs_with_sizes
        .values()
        .filter(|dir_size| **dir_size <= upper_bound)
        .sum())
}

pub fn p2(file: &str) -> Result<u32> {
    let navigations = &file[2..];
    let files_with_sizes = parse_files_with_sizes(navigations)?;

    let dirs_with_sizes = get_dir_sizes(&files_with_sizes);

    let total_space = 70_000_000u32;
    let total_used_space = *dirs_with_sizes.get(&PathBuf::from("/")).unwrap();
    let total_available_space = total_space - total_used_space;
    let total_to_free_up = 30_000_000u32;
    let left_to_free_up = total_to_free_up - total_available_space;

    Ok(*dirs_with_sizes
        .values()
        .filter(|dir_size| **dir_size >= left_to_free_up)
        .sorted_unstable()
        .next()
        .context("At least one directory")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let inp = include_str!("../inputs/d7/test.txt");
        assert_eq!(p1(inp).unwrap(), 95_437);
    }
    #[test]
    fn real_p1() {
        let inp = include_str!("../inputs/d7/real.txt");
        assert_eq!(p1(inp).unwrap(), 1_077_191);
    }
    #[test]
    fn test_p2() {
        let inp = include_str!("../inputs/d7/test.txt");
        assert_eq!(p2(inp).unwrap(), 24_933_642);
    }
    #[test]
    #[ignore]
    fn real_p2() {
        let inp = include_str!("../inputs/d7/real.txt");
        assert_eq!(p2(inp).unwrap(), 5_649_896);
    }
}
