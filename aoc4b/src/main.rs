use std::ops::Range;

fn parse_range(s: &str) -> Range<i32> {
    let (l, r) = s.split_once('-').unwrap();
    Range {
        start: l.parse().unwrap(),
        end: r.parse().unwrap(),
    }
}

fn parse_ranges(s: &str) -> (Range<i32>, Range<i32>) {
    let (l, r) = s.split_once(',').unwrap();
    (parse_range(l), parse_range(r))
}

fn overlapping(l: &Range<i32>, r: &Range<i32>) -> bool {
    (l.start <= r.start && l.end >= r.start)
        || (l.start <= r.end && l.end >= r.end)
        || (r.start <= l.start && r.end >= l.start)
        || (r.start <= l.end && r.end >= l.end)
}

fn overlaps<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    lines
        .map(|l| parse_ranges(l.as_ref()))
        .filter(|(l, r)| overlapping(l, r))
        .count()
}

fn main() {
    let overlaps = overlaps(
        std::io::stdin()
            .lines()
            .map(|l| l.expect("Couldn't read line")),
    );
    println!("Result: {overlaps:?}");
}

#[cfg(test)]
mod test {
    use crate::overlaps;

    #[test]
    fn example() {
        let input = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "#
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0);

        assert_eq!(overlaps(input), 4);
    }
}
