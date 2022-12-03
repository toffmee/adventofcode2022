use std::{collections::HashMap, fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("../inputs/day02.txt");

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
    let winning_combinations = vec!["A Y", "B Z", "C X"];
    let draw_combinations = vec!["A X", "B Y", "C Z"];
    let points_for_shape = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let mut total_score = 0;
    for line in data.lines() {
        total_score += points_for_shape[line.split(" ").last().unwrap()];
        if winning_combinations.contains(&line) {
            total_score += 6;
        } else if draw_combinations.contains(&line) {
            total_score += 3;
        }
    }
    total_score
}

fn part2(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    let winning_combinations = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let draw_combinations = HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")]);
    let losing_combinations = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let points_for_shape = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let mut total_score = 0;
    for line in data.lines() {
        let player1 = line.split(" ").nth(0).unwrap();
        let player2 = line.split(" ").nth(1).unwrap();
        if player2  == "X" {
            total_score += points_for_shape[losing_combinations[player1]];
        } else if player2 == "Y" {
            total_score += points_for_shape[draw_combinations[player1]];
            total_score += 3;
        } else if player2 == "Z" {
            total_score += points_for_shape[winning_combinations[player1]];
            total_score += 6;
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day02.txt")), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day02.txt")), 12);
    }
}
