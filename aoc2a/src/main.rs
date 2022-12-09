#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
use std::{ops::Add, str::FromStr};

use Play::*;

struct Match {
    theirs: Play,
    mine: Play,
}

#[derive(Debug, Default)]
struct Scores {
    theirs: i32,
    mine: i32,
}

impl Add for Scores {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            theirs: self.theirs + rhs.theirs,
            mine: self.mine + rhs.mine,
        }
    }
}

#[derive(Clone, Copy)]
enum MatchStatus {
    Win,
    Lose,
    Draw,
}
use MatchStatus::*;

impl Play {
    fn from_theirs(c: char) -> Option<Play> {
        match c {
            'A' => Some(Rock),
            'B' => Some(Paper),
            'C' => Some(Scissors),
            _ => None,
        }
    }

    fn from_mine(c: char) -> Option<Play> {
        match c {
            'X' => Some(Rock),
            'Y' => Some(Paper),
            'Z' => Some(Scissors),
            _ => None,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Match {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Match {
            theirs: Play::from_theirs(s.chars().nth(0).ok_or("Wrong size")?)
                .ok_or("Wrong theirs")?,
            mine: Play::from_mine(s.chars().nth(2).ok_or("Wrong size")?).ok_or("Wrong Mine")?,
        })
    }
}

impl Match {
    fn status(&self) -> MatchStatus {
        match (self.mine, self.theirs) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }

    fn score(&self) -> Scores {
        match self.status() {
            Draw => Scores {
                mine: self.mine.score() + 3,
                theirs: self.theirs.score() + 3,
            },
            Win => Scores {
                mine: self.mine.score() + 6,
                theirs: self.theirs.score(),
            },
            Lose => Scores {
                mine: self.mine.score(),
                theirs: self.theirs.score() + 6,
            },
        }
    }
}

fn score_all(input: impl IntoIterator<Item = String>) -> Scores {
    input
        .into_iter()
        .flat_map(|str| str.parse())
        .map(|str: Match| str.score())
        .fold(Default::default(), |acc, next| acc + next)
}

fn main() {
    let result = score_all(std::io::stdin().lines().flatten());
    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use crate::score_all;

    #[test]
    fn example() {
        let lines = ["A Y", "B X", "C Z"].map(String::from);
        let result = score_all(lines);
        assert_eq!(result.mine, 15);
        assert_eq!(result.theirs, 15);
    }
}
