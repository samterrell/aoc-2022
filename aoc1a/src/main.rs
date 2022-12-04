fn load(lines: impl Iterator<Item = impl Into<String>>) -> Vec<i32> {
    let mut elves = vec![];
    let mut current_elf = 0;
    for line in lines {
        if let Ok(calories) = line.into().parse::<i32>() {
            current_elf += calories;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }
    elves.push(current_elf);
    elves
}

fn max_calories(elves: Vec<i32>) -> Option<i32> {
    Some(*elves.iter().max()?)
}

fn main() {
    let elves = load(std::io::stdin().lines().into_iter().map(|l| l.unwrap()));
    let calories = max_calories(elves).unwrap();
    println!("{calories}");
}

#[cfg(test)]
mod tests {
    use crate::max_calories;

    #[test]
    fn example() {
        let data = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];
        let elves = super::load(data.into_iter());
        let max = max_calories(elves);
        assert_eq!(max, Some(24000))
    }
}
