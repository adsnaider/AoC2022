use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Hand {
    opponent: RPS,
    you: RPS,
}

impl RPS {
    pub fn wins_against(&self) -> Self {
        match *self {
            RPS::Rock => RPS::Scissor,
            RPS::Paper => RPS::Rock,
            RPS::Scissor => RPS::Paper,
        }
    }

    pub fn loses_against(&self) -> Self {
        match *self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissor,
            RPS::Scissor => RPS::Rock,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent = match s.chars().nth(0).unwrap() {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissor,
            _ => return Err(()),
        };
        let you = match s.chars().nth(2).unwrap() {
            'X' => opponent.wins_against(),
            'Y' => opponent,
            'Z' => opponent.loses_against(),
            _ => return Err(()),
        };
        Ok(Self { opponent, you })
    }
}

impl Hand {
    pub fn points(&self) -> usize {
        let shape_score = match self.you {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        };
        let winning_score = match (self.you, self.opponent) {
            (RPS::Rock, RPS::Rock) => 3,
            (RPS::Rock, RPS::Paper) => 0,
            (RPS::Rock, RPS::Scissor) => 6,

            (RPS::Paper, RPS::Rock) => 6,
            (RPS::Paper, RPS::Paper) => 3,
            (RPS::Paper, RPS::Scissor) => 0,

            (RPS::Scissor, RPS::Rock) => 0,
            (RPS::Scissor, RPS::Paper) => 6,
            (RPS::Scissor, RPS::Scissor) => 3,
        };
        winning_score + shape_score
    }
}

fn main() {
    let points = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|line| Hand::from_str(&line.unwrap()).unwrap())
        .fold(0usize, |points, hand| points + hand.points());

    println!("Final score is: {points}");
}
