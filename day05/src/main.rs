use itertools::Itertools;
use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("../inputs/day05.txt");

    let start = Instant::now();
    println!("Part 1 answer: {} in {:?}", part1(path), start.elapsed());

    let start = Instant::now();
    println!("Part 2 answer: {} in {:?}", part2(path), start.elapsed());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn parse_input(input: &Path) -> Option<(Vec<Vec<char>>, Vec<Vec<i32>>)> {
    let data = read_file(input).unwrap();
    let (left, instructions_str) = data.split_once("\n\n")?;
    let (stacks_str, platforms) = left.rsplit_once('\n')?;
    let num_stacks = platforms.split_whitespace().last()?.parse().ok()?;
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stacks_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let second = chunk.nth(1)?;
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    let mut instructions = Vec::<Vec<i32>>::new();
    for line in instructions_str.lines() {
        let parts: Vec<i32> = line
            .split(" ")
            .filter(|part| part.chars().all(|c| c.is_numeric()))
            .map(|part| part.parse::<i32>().unwrap())
            .collect();

        instructions.push(parts);
    }

    Some((stacks, instructions))
}

fn part1(input: &Path) -> String {
    let (mut stacks, instructions) = parse_input(input).unwrap();
    for instruction in instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        for _ in 0..amount as usize {
            if let Some(removed) = stacks[from as usize].pop() {
                stacks[to as usize].push(removed);
            }
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

fn part2(input: &Path) -> String {
    let (mut stacks, instructions) = parse_input(input).unwrap();
    for instruction in instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let from_stack_len = stacks[from as usize].len();
        let removed = stacks[from as usize].split_off(from_stack_len - amount as usize);
        stacks[to as usize].extend(removed);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day05.txt")), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day05.txt")), "MCD");
    }
}
