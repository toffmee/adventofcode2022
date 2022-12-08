use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("../inputs/day06.txt");

    let start = Instant::now();
    println!("Part 1 answer: {} in {:?}", part1(path), start.elapsed());

    let start = Instant::now();
    println!("Part 2 answer: {} in {:?}", part2(path), start.elapsed());
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn part1(input: &Path) -> i32 {
    let data = read_file(input).unwrap();
    let mut last_four: Vec<char> = Vec::new();

    for (i, character) in data.chars().enumerate() {
        last_four.push(character);
        if last_four.len() >= 4
            && last_four[0] != last_four[1]
            && last_four[0] != last_four[2]
            && last_four[0] != last_four[3]
            && last_four[1] != last_four[2]
            && last_four[1] != last_four[3]
            && last_four[2] != last_four[3]
        {
            return i as i32;
        }
        if last_four.len() > 4 {
            last_four.remove(0);
        }
    }
    0
}

fn part2(input: &Path) -> i32 {
    let data = read_file(input).unwrap();
    let mut last_fourteen: Vec<char> = Vec::new();

    for (i, character) in data.chars().enumerate() {
        last_fourteen.push(character);
        if last_fourteen.len() >= 14 {
            let mut is_marker = true;
            for j in 0..13 {
                for k in (j + 1)..14 {
                    if last_fourteen[j] == last_fourteen[k] {
                        is_marker = false;
                        break;
                    }
                }
                if !is_marker {
                    break;
                }
            }
            if is_marker {
                return i as i32;
            }
        }

        if last_fourteen.len() > 14 {
            last_fourteen.remove(0);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day06.txt")), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day06.txt")), 19);
    }
}
