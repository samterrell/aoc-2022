#[derive(Debug)]
enum Error {
    InvalidItem,
    InvalidLength,
    NoMatch,
}

#[derive(Debug, Clone, Copy)]
struct Item(u8);
impl Item {
    fn priority(self) -> u8 {
        self.0
    }
}

impl TryFrom<char> for Item {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' => Ok(Item(c as u8 - 'a' as u8 + 1)),
            'A'..='Z' => Ok(Item(c as u8 - 'A' as u8 + 27)),
            _ => Err(Error::InvalidItem),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BitmapSet(u64);
impl BitmapSet {
    fn new() -> Self {
        BitmapSet(0)
    }

    fn insert(&mut self, item: Item) {
        self.0 |= 1 << item.priority()
    }

    fn contains(self, item: Item) -> bool {
        (self.0 & 1 << item.priority()) != 0
    }
}

fn overlap(contents: &str) -> Result<Item, Error> {
    let len = contents.len();
    if len & 1 > 0 {
        return Err(Error::InvalidLength);
    }
    let (left, right) = contents.split_at(len / 2);
    let mut pocket = BitmapSet::new();
    for c in left.chars() {
        pocket.insert(c.try_into()?);
    }
    for c in right.chars() {
        let item = c.try_into()?;
        if pocket.contains(item) {
            return Ok(item);
        }
    }
    Err(Error::NoMatch)
}

fn main() {
    let value: u64 = std::io::stdin()
        .lines()
        .map(|l| l.expect("Reading line"))
        .map(|l| overlap(&l).expect("Shared item"))
        .map(|result| result.priority() as u64)
        .sum();
    println!("Result: {value}");
}

#[cfg(test)]
mod test {
    use crate::overlap;

    #[test]
    fn example() {
        assert_eq!(overlap("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap().priority(), 16);
        assert_eq!(
            overlap("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
                .unwrap()
                .priority(),
            38
        );
        assert_eq!(overlap("PmmdzqPrVvPwwTWBwg").unwrap().priority(), 42);
        assert_eq!(
            overlap("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")
                .unwrap()
                .priority(),
            22
        );
        assert_eq!(overlap("ttgJtRGJQctTZtZT").unwrap().priority(), 20);
        assert_eq!(overlap("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap().priority(), 19);
    }
}
