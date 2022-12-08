use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("../inputs/day04.txt");

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
    let mut fully_contained_sections = 0;
    for line in data.lines() {
        let assignment_sections: Vec<i32> = line
            .split(",")
            .map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()))
            .flatten()
            .collect();
        if assignment_sections[0] <= assignment_sections[2]
            && assignment_sections[3] <= assignment_sections[1]
        {
            // println!("{}-{} is fully contained in {}-{}", assignment_sections[2], assignment_sections[3], assignment_sections[0], assignment_sections[1]);
            fully_contained_sections += 1;
        } else if assignment_sections[2] <= assignment_sections[0]
            && assignment_sections[1] <= assignment_sections[3]
        {
            // println!("{}-{} is fully contained in {}-{}", assignment_sections[0], assignment_sections[1], assignment_sections[2], assignment_sections[3]);
            fully_contained_sections += 1;
        }
    }
    fully_contained_sections
}

fn part2(input: &Path) -> i64 {
    let data = read_file(input).unwrap();
    let mut fully_contained_sections = 0;
    for line in data.lines() {
        let assignment_sections: Vec<i32> = line
            .split(",")
            .map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()))
            .flatten()
            .collect();
        if assignment_sections[0] >= assignment_sections[2]
            && assignment_sections[0] <= assignment_sections[3]
        {
            fully_contained_sections += 1;
            // println!("{}-{} start overlaps with {}-{}", assignment_sections[0], assignment_sections[1], assignment_sections[2], assignment_sections[3]);
        } else if assignment_sections[1] >= assignment_sections[2]
            && assignment_sections[1] <= assignment_sections[3]
        {
            fully_contained_sections += 1;
            // println!("{}-{} end overlaps with {}-{}", assignment_sections[0], assignment_sections[1], assignment_sections[2], assignment_sections[3]);
        } else if assignment_sections[2] >= assignment_sections[0]
            && assignment_sections[2] <= assignment_sections[1]
        {
            fully_contained_sections += 1;
            // println!("{}-{} start overlaps with {}-{}", assignment_sections[2], assignment_sections[3], assignment_sections[1], assignment_sections[2]);
        } else if assignment_sections[3] >= assignment_sections[0]
            && assignment_sections[3] <= assignment_sections[1]
        {
            fully_contained_sections += 1;
            // println!("{}-{} end overlaps with {}-{}", assignment_sections[2], assignment_sections[3], assignment_sections[1], assignment_sections[2]);
        }
    }
    fully_contained_sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_part1() {
        assert_eq!(part1(Path::new("../tests/day04.txt")), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(Path::new("../tests/day04.txt")), 4);
    }
}
