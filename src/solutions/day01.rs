use crate::PuzzleSolver;

pub struct Day01 {
    _lines: Vec<String>,
}

impl PuzzleSolver for Day01 {
    fn with_input(input: String) -> Self {
        Self {
            _lines: input.lines().map(|l| l.to_string()).collect(),
        }
    }

    fn part1(&self) -> i128 {
        // Implement a solution to part 1 here
        todo!()
    }

    fn part2(&self) -> i128 {
        // Implement a solution to part 1 here
        todo!()
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(1, 0, 0);
