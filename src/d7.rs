use std::collections::HashMap;

use itertools::Itertools;

struct Directory {
    subdirs: Vec<Box<str>>,
    own_size: u32,
}

impl Directory {
    fn build(subdirs: Vec<Box<str>>, own_size: u32) -> Self {
        Self { subdirs, own_size }
    }
}

pub fn p1(file: &str) -> u32 {
    let navigations = &file[2..];
    let mut dir_stack = Vec::new();
    let mut filesystem: HashMap<&str, Directory> = HashMap::new();

    for input_and_output in navigations.split("\n$ ") {
        match input_and_output.split_whitespace().collect_vec().as_slice() {
            ["cd", dir_name] => match *dir_name {
                ".." => drop(dir_stack.pop()),
                dir_name => dir_stack.push(dir_name),
            },
            _ => {
                let (_input, output) = input_and_output.split_once("\n").unwrap();
                let mut subdirs = Vec::new();
                let mut own_size = 0;
                for line in output.lines() {
                    match line.split_once(" ").unwrap() {
                        ("dir", dir_name) => subdirs.push(Box::from(dir_name)),
                        (file_size, _file_name) => own_size += file_size.parse::<u32>().unwrap(),
                    }
                }
                let dir_name = dir_stack.last().unwrap();
                let dir = Directory::build(subdirs, own_size);
                filesystem.entry(dir_name).or_insert(dir);
            }
        }
    }

    fn get_dir_size<'a>(dir_name: &str, filesystem: &'a HashMap<&str, Directory>) -> u32 {
        let dir = &filesystem[dir_name];
        let size = dir.own_size
            + if let [] = dir.subdirs[..] {
                0
            } else {
                dir.subdirs
                    .iter()
                    .map(|subdir_name| get_dir_size(subdir_name, filesystem))
                    .sum()
            };
        size
    }

    filesystem
        .keys()
        .map(|dir_name| get_dir_size(dir_name, &filesystem))
        .filter(|dir_size| *dir_size <= 100000)
        .sum()
}

pub fn p2(file: &str) -> u32 {
    todo!()
}
