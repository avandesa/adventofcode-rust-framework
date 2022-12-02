mod solutions;
#[cfg(test)]
mod test_helpers;

use solutions::*;

/// Construct the appropriate solver for the given day
pub fn build_solver(day: u8, input: String) -> impl PuzzleSolver {
    match day {
        1 => Day01::with_input(input),
        // As you produce solutions for other days, add them here
        _ => todo!("Day {day} is not implemented yet"),
    }
}

/// Represents structs that take an input and produce two-part solutions to an AdventOfCode puzzle
pub trait PuzzleSolver {
    /// Constructor; the solver may do whatever it wants with the input to build data structures
    /// needed to solve the day's puzzles
    fn with_input(input: String) -> Self;

    /// Produce a solution for part 1 of the day's puzzle
    fn part1(&self) -> i128 {
        todo!()
    }

    /// Produce a solution for part 2 of the day's puzzle
    fn part2(&self) -> i128 {
        todo!()
    }
}
