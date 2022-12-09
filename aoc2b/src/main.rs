use std::{ops::Add, str::FromStr};

#[derive(Clone, Copy, PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

use Play::*;

impl Play {
    fn from_char(c: char) -> Result<Play, &'static str> {
        match c {
            'A' => Ok(Rock),
            'B' => Ok(Paper),
            'C' => Ok(Scissors),
            _ => Err("Invalid play."),
        }
    }

    fn compare(&self, rhs: &Play) -> MatchStatus {
        if self == rhs {
            Draw
        } else if self.dec() == *rhs {
            Win
        } else {
            Lose
        }
    }

    fn inc(&self) -> Play {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn dec(&self) -> Play {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

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

impl FromStr for Match {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let theirs = Play::from_char(s.chars().nth(0).ok_or("Wrong size")?)?;
        let mine = match s.chars().nth(2).ok_or("Wrong size")? {
            'X' => theirs.dec(),
            'Y' => theirs.clone(),
            'Z' => theirs.inc(),
            _ => return Err("Invalid response"),
        };
        Ok(Match { theirs, mine })
    }
}

impl Match {
    fn score(&self) -> Scores {
        match self.mine.compare(&self.theirs) {
            Draw => Scores {
                mine: self.mine as i32 + 3,
                theirs: self.theirs as i32 + 3,
            },
            Win => Scores {
                mine: self.mine as i32 + 6,
                theirs: self.theirs as i32,
            },
            Lose => Scores {
                mine: self.mine as i32,
                theirs: self.theirs as i32 + 6,
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
        assert_eq!(result.mine, 12);
        assert_eq!(result.theirs, 15);
    }
}
