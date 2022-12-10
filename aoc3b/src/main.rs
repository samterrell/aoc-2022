use std::str::FromStr;

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

impl TryFrom<BitmapSet> for Item {
    type Error = Error;

    fn try_from(value: BitmapSet) -> Result<Self, Self::Error> {
        let mut v = 1;
        for i in 1..=53 {
            if v == value.0 {
                return Ok(Item(i));
            }
            v <<= 1;
        }
        return Err(Error::NoMatch);
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
        self.0 |= 1 << (item.priority() - 1)
    }

    fn union(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    fn intersect(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[derive(Debug)]
struct Pocket(BitmapSet);

impl FromStr for Pocket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = BitmapSet::new();
        for c in s.chars() {
            set.insert(c.try_into()?);
        }
        Ok(Self(set))
    }
}

#[derive(Debug)]
struct Rucksack(Pocket, Pocket);

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len & 1 > 0 {
            return Err(Error::InvalidLength);
        }
        let (left, right) = s.split_at(len / 2);
        Ok(Rucksack(left.parse()?, right.parse()?))
    }
}

impl Rucksack {
    fn full_set(&self) -> BitmapSet {
        self.0 .0.union(self.1 .0)
    }
}

#[derive(Debug)]
struct Group([Rucksack; 3]);

impl Group {
    fn badge(&self) -> Result<Item, Error> {
        self.0
            .iter()
            .map(|v| v.full_set())
            .reduce(|l, r| l.intersect(r))
            .ok_or(Error::InvalidLength)?
            .to_owned()
            .try_into()
    }
}

struct GroupIter<I: Iterator<Item = T>, T: AsRef<str>>(I);
impl<I: Iterator<Item = T>, T: AsRef<str>> Iterator for GroupIter<I, T> {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Group([
            self.0.next()?.as_ref().parse().ok()?,
            self.0.next()?.as_ref().parse().ok()?,
            self.0.next()?.as_ref().parse().ok()?,
        ]))
    }
}
impl<I: Iterator<Item = T>, T: AsRef<str>> From<I> for GroupIter<I, T> {
    fn from(inner: I) -> Self {
        Self(inner)
    }
}

fn main() {
    let iter: GroupIter<_, _> = std::io::stdin().lines().map_while(|v| v.ok()).into();
    let value: u32 = iter
        .map(|g| g.badge().expect("Couldn't get badge").priority() as u32)
        .sum();
    println!("Result: {value}");
}

#[cfg(test)]
mod test {
    use crate::GroupIter;

    #[test]
    fn example() {
        let data = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let mut v: GroupIter<_, _> = data.iter().into();
        assert_eq!(
            v.next()
                .expect("a group")
                .badge()
                .expect("badge")
                .priority(),
            18
        );
        assert_eq!(
            v.next()
                .expect("a group")
                .badge()
                .expect("badge")
                .priority(),
            52
        );
    }
}
