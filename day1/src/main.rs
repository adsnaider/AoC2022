use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let input = Path::new("input");
    let input = File::open(input).unwrap();
    let topn: usize = BufReader::new(input)
        .lines()
        .map(|line| line.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty_line, group)| {
            if !is_empty_line {
                Some(group.map(|line| line.parse().unwrap()))
            } else {
                None
            }
        })
        .map(|group| group.fold(0usize, |l: usize, r: usize| l + r))
        .sorted()
        .rev()
        .take(3)
        .fold(0usize, |l, r| l + r);

    println!("3 Elves with most calories has: {topn} cal");
}
