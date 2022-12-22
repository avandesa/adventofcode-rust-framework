use crate::PuzzleSolver;
use std::collections::{HashSet, VecDeque};

pub struct Day06 {
    input: Vec<char>,
}

impl PuzzleSolver for Day06 {
    fn with_input(input: String) -> Self {
        Self {
            input: input.chars().collect(),
        }
    }

    fn part1(&self) -> String {
        find_signal(&self.input, 4).expect("No signal").to_string()
    }

    fn part2(&self) -> String {
        find_signal(&self.input, 14).expect("No signal").to_string()
    }
}

fn find_signal(input: &[char], length: usize) -> Option<usize> {
    let mut ring = Ring::new(length);
    for (i, c) in input.iter().enumerate() {
        ring.push(*c);
        if ring.all_unique() {
            return Some(i + 1);
        }
    }
    None
}

#[derive(Debug)]
struct Ring {
    capacity: usize,
    // Ring buffer for cycling characters
    buffer: VecDeque<char>,
    // Re-use a hash set for each uniqueness check so we don't have to constantly allocate
    hash_set: HashSet<char>,
}

impl Ring {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
            hash_set: HashSet::with_capacity(capacity),
        }
    }

    fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    fn push(&mut self, c: char) {
        if self.is_full() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(c);
    }

    fn all_unique(&mut self) -> bool {
        if !self.is_full() {
            return false;
        }

        self.hash_set.clear();
        for c in &self.buffer {
            if self.hash_set.contains(c) {
                return false;
            }
            self.hash_set.insert(*c);
        }

        true
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(6, "7", "19");
