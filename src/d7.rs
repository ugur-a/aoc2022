use std::{collections::HashMap, path::PathBuf};

use itertools::Itertools;

fn parse_files_from_navigation(navigations: &str) -> HashMap<PathBuf, u32> {
    let mut current_path = PathBuf::new();
    let mut files_with_sizes: HashMap<PathBuf, u32> = HashMap::new();

    for input_and_output in navigations.split("\n$ ") {
        if let ["cd", dir_name] = input_and_output.split_whitespace().collect_vec().as_slice() {
            match *dir_name {
                ".." => current_path = current_path.parent().unwrap().to_path_buf(),
                _ => current_path.push(dir_name),
            }
        } else {
            let (_input, output) = input_and_output.split_once("\n").unwrap();
            for line in output.lines() {
                match line.split_once(" ").unwrap() {
                    ("dir", _dir_name) => continue,
                    (file_size, file_name) => {
                        let file_size_parsed = file_size.parse::<u32>().unwrap();
                        let file_path = current_path.join(file_name);
                        files_with_sizes.insert(file_path, file_size_parsed);
                    }
                }
            }
        }
    }
    files_with_sizes
}

fn get_dir_sizes(files_with_sizes: HashMap<PathBuf, u32>) -> HashMap<PathBuf, u32> {
    let mut dirs_with_sizes: HashMap<PathBuf, u32> = HashMap::new();

    for (file_path, file_size) in &files_with_sizes {
        for ancestor in file_path.ancestors().skip(1) {
            dirs_with_sizes
                .entry(ancestor.to_path_buf())
                .and_modify(|dir_size| *dir_size += file_size)
                .or_insert(*file_size);
        }
    }
    dirs_with_sizes
}

pub fn p1(file: &str) -> u32 {
    let upper_bound = 100_000u32;

    let navigations = &file[2..];
    let files_with_sizes = parse_files_from_navigation(navigations);

    let dirs_with_sizes = get_dir_sizes(files_with_sizes);

    dirs_with_sizes
        .values()
        .filter(|dir_size| **dir_size <= upper_bound)
        .sum()
}

pub fn p2(file: &str) -> u32 {
    let navigations = &file[2..];
    let files_with_sizes = parse_files_from_navigation(navigations);

    let dirs_with_sizes = get_dir_sizes(files_with_sizes);

    let total_space = 70_000_000u32;
    let total_used_space = *dirs_with_sizes.get(&PathBuf::from("/")).unwrap();
    let total_available_space = total_space - total_used_space;
    let total_to_free_up = 30_000_000u32;
    let left_to_free_up = total_to_free_up - total_available_space;

    *dirs_with_sizes
        .values()
        .filter(|dir_size| **dir_size >= left_to_free_up)
        .sorted_unstable()
        .next()
        .unwrap()
}
