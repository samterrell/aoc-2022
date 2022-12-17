use std::str::FromStr;

#[derive(Debug)]
enum Error {
    Underflow,
    InvalidCommand,
}

struct State {
    stacks: Vec<Vec<char>>,
}

struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl State {
    fn parse(lines: &mut impl Iterator<Item = impl Into<String>>) -> Self {
        let mut data: Vec<Vec<char>> = vec![];
        while let Some(line) = lines.next() {
            let line: String = line.into();
            if line.trim().len() == 0 {
                break;
            }
            data.push(line.chars().collect());
        }
        let mut stacks = vec![];
        if data.len() == 0 {
            return Self { stacks };
        }
        for (i, c) in data.last().unwrap().iter().enumerate() {
            if !c.is_alphanumeric() {
                continue;
            }
            let mut stack = vec![];
            for l in data.iter().rev().skip(1) {
                let Some(c2) = l.get(i) else {
                    break;
                };
                if c2.is_alphanumeric() {
                    stack.push(*c2);
                }
            }
            stacks.push(stack);
        }
        Self { stacks }
    }

    fn peek(&self) -> String {
        let mut result = String::with_capacity(self.stacks.len());
        for stack in &self.stacks {
            result.push(*stack.last().unwrap_or(&' '));
        }
        result
    }

    fn pop_from(&mut self, s: usize) -> Option<char> {
        self.stacks.get_mut(s)?.pop()
    }

    fn push_to(&mut self, s: usize, c: char) -> Result<(), Error> {
        match self.stacks.get_mut(s) {
            Some(stack) => Ok(stack.push(c)),
            None => Err(Error::Underflow),
        }
    }

    fn execute(&mut self, cmd: Command) -> Result<(), Error> {
        for _i in 0..cmd.count {
            let Some(c) = self.pop_from(cmd.from - 1) else {
                return Err(Error::Underflow);
            };
            self.push_to(cmd.to - 1, c)?;
        }
        Ok(())
    }
}

fn expect<'a>(words: &mut impl Iterator<Item = &'a str>, word: &'static str) -> Result<(), Error> {
    let Some(next) = words.next() else {
        return Err(Error::InvalidCommand);
    };
    if !next.eq_ignore_ascii_case(word) {
        return Err(Error::InvalidCommand);
    }
    Ok(())
}

fn get_value<'a>(words: &mut impl Iterator<Item = &'a str>) -> Result<usize, Error> {
    words
        .next()
        .unwrap_or("")
        .parse()
        .map_err(|_| Error::InvalidCommand)
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        expect(&mut words, "move")?;
        let count = get_value(&mut words)?;
        expect(&mut words, "from")?;
        let from = get_value(&mut words)?;
        expect(&mut words, "to")?;
        let to = get_value(&mut words)?;
        match words.next() {
            None => Ok(Self { from, to, count }),
            _ => Err(Error::InvalidCommand),
        }
    }
}

fn main() {
    let mut lines = std::io::stdin()
        .lines()
        .map(|l| l.expect("Can't read line."));
    let mut state = State::parse(&mut lines);
    lines.for_each(|l| {
        state
            .execute(l.parse().expect("Invalid Command"))
            .expect("Command could not execute")
    });
    println!("{}", state.peek());
}

#[cfg(test)]
mod test {
    use crate::{Command, State};

    #[test]
    fn example() {
        let mut data = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .lines()
            .into_iter();
        let mut state = State::parse(&mut data);
        assert_eq!(state.peek(), "NDP");
        let cmd: Command = data.next().unwrap().parse().expect("Invalid Command");
        assert_eq!(cmd.count, 1);
        assert_eq!(cmd.from, 2);
        assert_eq!(cmd.to, 1);
        state.execute(cmd).expect("Command did not execute");
        assert_eq!(state.peek(), "DCP");
        let cmd: Command = data.next().unwrap().parse().expect("Invalid Command");
        assert_eq!(cmd.count, 3);
        assert_eq!(cmd.from, 1);
        assert_eq!(cmd.to, 3);
        state.execute(cmd).expect("Command did not execute");
        assert_eq!(state.peek(), " CZ");
        let cmd: Command = data.next().unwrap().parse().expect("Invalid Command");
        assert_eq!(cmd.count, 2);
        assert_eq!(cmd.from, 2);
        assert_eq!(cmd.to, 1);
        state.execute(cmd).expect("Command did not execute");
        assert_eq!(state.peek(), "M Z");
        let cmd: Command = data.next().unwrap().parse().expect("Invalid Command");
        assert_eq!(cmd.count, 1);
        assert_eq!(cmd.from, 1);
        assert_eq!(cmd.to, 2);
        state.execute(cmd).expect("Command did not execute");
        assert_eq!(state.peek(), "CMZ");
    }
}
