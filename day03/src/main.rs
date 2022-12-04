use std::{fs, path::Path, time::Instant, collections::HashSet};

fn main() {
    let path = Path::new("../inputs/day03.txt");

    let start = Instant::now();
    println!("Part 1 answer: {} in {:?}", part1(path), start.elapsed());

    let start = Instant::now();
    println!("Part 2 answer: {} in {:?}", part2(path), start.elapsed());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn calculate_item_priority(item_type: char) -> i64 {
    let priority: i64 = if item_type.is_lowercase() {
        item_type as i64 - 'a' as i64 + 1
    } else {
        item_type as i64 - 'A' as i64 + 27
    };
    priority
}

fn split_rucksack(rucksack: &str) -> (String, String) {
    let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
    (first_compartment.to_string(), second_compartment.to_string())
}

fn shared_items(rucksacks: Vec<&str>) -> Vec<char> {
    let mut set: HashSet<char> = rucksacks[0].chars().collect();
    for string in rucksacks.iter().skip(1) {
        let set2: HashSet<char> = string.chars().collect();
        set = set.intersection(&set2).copied().collect();
    }
    set.into_iter().collect()
}

fn part1(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    println!("A priority: {}", calculate_item_priority('A'));
    let mut sum_of_priorities = 0;
    for line in data.lines() {
        let (first_compartment, second_compartment) = split_rucksack(line);
        let shared_items = shared_items(vec![&first_compartment, &second_compartment]);
        for item in shared_items {
            sum_of_priorities += calculate_item_priority(item);
        }
    }
    sum_of_priorities
}

fn part2(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    let mut sum_of_priorities = 0;
    for i in (0..data.lines().count()).step_by(3) {
        let lines: Vec<&str> = data.lines().skip(i).take(3).collect();
        let shared_items = shared_items(vec![&lines[0], &lines[1], &lines[2]]);
        for item in shared_items {
            sum_of_priorities += calculate_item_priority(item);
        }
    }
    sum_of_priorities
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day03.txt")), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day03.txt")), 70);
    }
}
