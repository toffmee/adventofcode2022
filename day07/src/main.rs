use std::{fs, path::Path, time::Instant};
use regex::Regex;

struct Directory {
    name: String,
    size: i32,
    parent: usize,
}

fn main() {
    let path = Path::new("../inputs/day07.txt");

    let start = Instant::now();
    println!("Part 1 answer: {} in {:?}", part1(path), start.elapsed());

    let start = Instant::now();
    println!("Part 2 answer: {} in {:?}", part2(path), start.elapsed());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}


fn parse_input(input: &str) -> Vec<Directory> {

    let mut directories = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];

    let root_directory_index = 0;
    let mut current_directory = root_directory_index;

    let directory_regex = Regex::new(r"\s*\$ cd (?P<folder>.+)").unwrap();
    let file_regex = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    for line in input.lines() {

        if line.is_empty() {
            continue;
        }

        if let Some(captures) = directory_regex.captures(line) {
            match &captures["folder"] {
                "/" => {
                    current_directory = root_directory_index;
                }
                ".." => {
                    current_directory = directories[current_directory].parent;
                }
                _ => {
                    let directory_name = &captures["folder"];
                    directories.push(Directory {
                        name: directory_name.to_string(),
                        size: 0,
                        parent: current_directory,
                    });
                    current_directory = directories.len() - 1;
                }
            }
        } else if let Some(captures) = file_regex.captures(line) {
            let size = captures["size"].parse::<i32>().unwrap();
            let mut path_index = current_directory;
            loop {
                directories[path_index].size += size;
                if path_index == root_directory_index {
                    break;
                }
                path_index = directories[path_index].parent;
            }
        }
    }
    return directories;
}

fn part1(input: &Path) -> i32 {
    let data = read_file(input).unwrap();

    let directories = parse_input(&data);
    let mut total_size = 0;
    for directory in &directories {
        if directory.size <= 100000 {
            total_size += directory.size;
        }

    }
    total_size
}

fn part2(input: &Path) -> i32 {
    let data = read_file(input).unwrap();
    let directories = parse_input(&data);

    let free_space = 70000000 - directories[0].size;
    let needs_to_be_freed = 30000000 - free_space;
    // println!("Free space: {} Needs to be freed: {}", free_space, needs_to_be_freed);
    let mut directory_to_delete = 0;
    for i in 0..directories.len(){
        // println!("Directory: {} Directory size: {} Parent: {}", directories[i].name, directories[i].size, directories[directories[i].parent].name);
        if directories[i].size  >= needs_to_be_freed {
            if directories[i].size < directories[directory_to_delete].size {
                directory_to_delete = i;
            }
        }
    }
    directories[directory_to_delete].size
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day07.txt")), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day07.txt")), 24933642);
    }
}
