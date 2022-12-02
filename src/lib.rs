mod solutions;
#[cfg(test)]
mod test_helpers;

use solutions::*;

/// Construct the appropriate solver for the given day
pub fn build_solver(day: u8, input: String) -> Box<dyn PuzzleSolver> {
    match day {
        1 => Box::new(Day01::with_input(input)),
        2 => Box::new(Day02::with_input(input)),
        // As you produce solutions for other days, add them here
        _ => todo!("Day {day} is not implemented yet"),
    }
}

/// Represents structs that take an input and produce two-part solutions to an AdventOfCode puzzle
pub trait PuzzleSolver {
    /// Constructor; the solver may do whatever it wants with the input to build data structures
    /// needed to solve the day's puzzles
    fn with_input(input: String) -> Self
    where
        Self: Sized;

    /// Produce a solution for part 1 of the day's puzzle
    fn part1(&self) -> String {
        todo!()
    }

    /// Produce a solution for part 2 of the day's puzzle
    fn part2(&self) -> String {
        todo!()
    }
}
