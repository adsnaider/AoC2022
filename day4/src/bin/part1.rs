use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    str::FromStr,
};

struct CleaningAssignment(Range<usize>, Range<usize>);

fn decode_range(s: &str) -> Range<usize> {
    let (start, end) = s.split_once('-').unwrap();
    let start = start.parse().unwrap();
    let end: usize = end.parse().unwrap();
    start..(end + 1)
}

impl CleaningAssignment {
    pub fn fully_contained(&self) -> bool {
        (self.0.contains(&self.1.start) && self.0.contains(&(self.1.end - 1)))
            || (self.1.contains(&self.0.start) && self.1.contains(&(self.0.end - 1)))
    }
}

impl FromStr for CleaningAssignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (elf1, elf2) = s.split_once(',').unwrap();
        Ok(CleaningAssignment(decode_range(elf1), decode_range(elf2)))
    }
}

fn main() {
    let fully_contained_assignments = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|line| CleaningAssignment::from_str(&line.unwrap()).unwrap())
        .filter(|assignment| assignment.fully_contained())
        .count();

    println!("There are {fully_contained_assignments} fully contained assignments");
}
