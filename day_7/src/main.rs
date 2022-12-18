use std::{collections::HashMap, str::Lines};

fn main() {
    let input = include_str!("../input.txt");

    let root = parse(input);
    let small_dirs_total = get_total_size_of_all_small_dirs(&root);

    println!("Small dirs total: {small_dirs_total}");

    let (_dir_to_delete, size) = get_dir_to_delete(&root);

    println!("Dir to delete size: {size}");
}

fn parse(input: &str) -> Directory {
    let mut lines = input.lines();

    let mut root = Directory::new();

    // Ignore first lines
    _ = lines.next(); // $ cd /
    _ = lines.next(); // $ ls

    parse_directory(&mut root, &mut lines);

    root
}

const DIR_PREFIX: &str = "dir ";
const CD_PREFIX: &str = "$ cd ";

fn parse_directory(parent_dir: &mut Directory, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            return;
        }

        if line == "$ ls" {
            continue;
        }

        if line.starts_with(DIR_PREFIX) {
            let child_name = line.strip_prefix(DIR_PREFIX).unwrap().to_owned();
            parent_dir.directories.insert(child_name, Directory::new());
            continue;
        }

        if line.starts_with(CD_PREFIX) {
            let child_name = line.strip_prefix(CD_PREFIX).unwrap().to_owned();
            parse_directory(parent_dir.directories.get_mut(&child_name).unwrap(), lines);
            continue;
        }

        let mut line_split = line.split(' ');
        parent_dir.files.push(File {
            size: line_split.next().unwrap().parse().unwrap(),
            name: String::from(line_split.next().unwrap()),
        });
    }
}

fn get_total_size_of_all_small_dirs(root: &Directory) -> u64 {
    Directories::new(root)
        .map(|dir| dir.get_size())
        .filter(|size| *size <= 100000)
        .sum()
}

const TOTAL_CAPACITY: u64 = 70000000;
const REQUIRED_SPACE: u64 = 30000000;
const DESIRED_USED: u64 = TOTAL_CAPACITY - REQUIRED_SPACE;

fn get_dir_to_delete(root: &Directory) -> (&Directory, u64) {
    let space_needed = root.get_size() - DESIRED_USED;

    Directories::new(root)
        .map(|dir| (dir, dir.get_size()))
        .filter(|(_dir, size)| *size >= space_needed)
        .min_by_key(|(_dir, size)| *size)
        .unwrap()
}

struct Directory {
    directories: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn get_size(&self) -> u64 {
        let files_size = self.files.iter().map(|file| file.size).sum::<u64>();
        let directories_size = self
            .directories
            .values()
            .map(Directory::get_size)
            .sum::<u64>();

        files_size + directories_size
    }
}

struct Directories<'a> {
    stack: Vec<(&'a Directory, usize)>,
}

impl<'a> Directories<'a> {
    fn new(start: &'a Directory) -> Self {
        Self {
            stack: vec![(start, 0)],
        }
    }
}

impl<'a> Iterator for Directories<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current_parent, child_index)) = self.stack.pop() {
            let dir = match current_parent.directories.values().nth(child_index) {
                Some(dir) => dir,
                None => continue,
            };

            self.stack.push((current_parent, child_index + 1));
            self.stack.push((dir, 0));

            return Some(dir);
        }

        None
    }
}

struct File {
    #[allow(dead_code)]
    name: String,
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let test_content = include_str!("../test_input.txt");
        let root = parse(test_content);

        assert_eq!(root.get_size(), 48381165);

        assert_eq!(Directories::new(&root).count(), 3);

        assert_eq!(get_total_size_of_all_small_dirs(&root), 95437);
    }

    #[test]
    fn example_part2() {
        let test_content = include_str!("../test_input.txt");
        let root = parse(test_content);

        let (_dir_to_delete, size) = get_dir_to_delete(&root);

        assert_eq!(size, 24933642);
    }
}
