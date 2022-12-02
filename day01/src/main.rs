use std::{path::Path, time::Instant, fs};

fn main() {
    let path = Path::new("../inputs/day01.txt");

    let start = Instant::now();
    println!("Part 1 answer: {} in {:?}", part1(path), start.elapsed());

    let start = Instant::now();
    println!("Part 2 answer: {} in {:?}", part2(path), start.elapsed());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn part1(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    let mut calories: Vec<i64> = data
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split("\n")
                .map(|item| item.parse::<i64>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();
    calories.reverse();
    calories[0]
}

fn part2(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    let mut calories: Vec<i64> = data
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split("\n")
                .map(|item| item.parse::<i64>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day01.txt")), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day01.txt")), 45000);
    }
}
