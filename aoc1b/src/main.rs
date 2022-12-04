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

struct MaxStack([i32; 3]);
impl MaxStack {
    fn new() -> Self {
        Self([0, 0, 0])
    }
    fn push(self, value: i32) -> Self {
        match self.0 {
            [_, _, l] if value < l => self,
            [h, m, _] if value < m => Self([h, m, value]),
            [h, m, _] if value < h => Self([h, value, m]),
            [h, m, _] => Self([value, h, m]),
        }
    }
    fn sum(self) -> i32 {
        self.0.iter().sum()
    }
}

fn top_three_calories_sum(elves: Vec<i32>) -> i32 {
    elves
        .iter()
        .fold(MaxStack::new(), |acc, x| acc.push(*x))
        .sum()
}

fn main() {
    let elves = load(std::io::stdin().lines().into_iter().map(|l| l.unwrap()));
    let calories = top_three_calories_sum(elves);
    println!("{calories}");
}

#[cfg(test)]
mod tests {
    use crate::{load, top_three_calories_sum};

    #[test]
    fn example() {
        let data = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];
        let elves = load(data.into_iter());
        let sum = top_three_calories_sum(elves);
        assert_eq!(sum, 45000)
    }
}
