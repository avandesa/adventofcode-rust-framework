use crate::PuzzleSolver;
use std::collections::HashSet;

pub struct Day03 {
    sacks: Vec<Rucksack>,
}

impl PuzzleSolver for Day03 {
    fn with_input(input: String) -> Self {
        let sacks: Vec<Rucksack> = input.lines().map(Rucksack::from_line).collect();
        Self { sacks }
    }

    fn part1(&self) -> String {
        self.sacks
            .iter()
            .map(|sack| sack.find_duplicate().priority() as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.sacks
            .chunks(3)
            .map(|group| Rucksack::find_badge(&group[0], &group[1], &group[2]).priority() as u32)
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn from_char(c: char) -> Self {
        if !c.is_ascii_alphabetic() {
            panic!("Invalid character: {}", c);
        }
        Self(c)
    }

    fn priority(&self) -> u8 {
        if self.0.is_ascii_lowercase() {
            self.0 as u8 - 'a' as u8 + 1
        } else {
            self.0 as u8 - 'A' as u8 + 27
        }
    }
}

#[derive(Debug, Clone)]
struct Compartment(HashSet<Item>);

impl Compartment {
    fn from_str(s: &str) -> Self {
        Self(s.chars().map(Item::from_char).collect())
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    total: HashSet<Item>,
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    fn from_line(line: &str) -> Self {
        assert_eq!(
            line.len() % 2,
            0,
            "Line is not an even number of characters"
        );
        let total = line.chars().map(Item::from_char).collect();
        let (left, right) = line.split_at(line.len() / 2);
        Self {
            total,
            left: Compartment::from_str(left),
            right: Compartment::from_str(right),
        }
    }

    fn find_duplicate(&self) -> Item {
        *self
            .left
            .0
            .intersection(&self.right.0)
            .nth(0)
            .expect("at least one duplicate")
    }

    fn find_badge(a: &Self, b: &Self, c: &Self) -> Item {
        *a.total
            .intersection(&b.total)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&c.total)
            .nth(0)
            .expect("at least one duplicate")
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(3, "157", "70");
