use crate::PuzzleSolver;

pub struct Day01 {
    bags: Vec<Bag>,
}

impl PuzzleSolver for Day01 {
    fn with_input(input: String) -> Self {
        let mut bags: Vec<Bag> = input.split("\n\n").map(Bag::from_str).collect();
        bags.sort_by(|a, b| b.0.cmp(&a.0));
        Self { bags }
    }

    fn part1(&self) -> String {
        self.bags[0].0.to_string()
    }

    fn part2(&self) -> String {
        self.bags
            .iter()
            .take(3)
            .map(|b| b.0)
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug)]
struct Bag(u32);

impl Bag {
    fn from_str(input: &str) -> Self {
        let items: Vec<u32> = input.lines().map(|n| n.parse().unwrap()).collect();
        let total = items.iter().sum();
        Self(total)
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(1, "24000", "45000");
