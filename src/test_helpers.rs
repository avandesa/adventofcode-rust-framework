// We'll get warnings if the macro is only ever called with one or two arguments
#![allow(dead_code, unused_imports)]

/// For a given day, try to load the 'short' input
pub fn load_short_input_for_day(day: u8) -> String {
    let path = format!("./inputs/day{:02}-short.txt", day);
    std::fs::read_to_string(&path).expect(&format!("Failed to open {path}"))
}

macro_rules! _test_ignored {
    ($day:literal, $part:literal) => {
        paste::paste! {
            #[test]
            #[ignore]
            fn [<_day $day _part $part _test_short_input>]() {}
        }
    };
}

macro_rules! _test {
    ($day:literal, $part:literal, $expected_val:expr) => {
        paste::paste! {
            #[test]
            fn [<_day $day _part $part _test_short_input>]() {
                let input = crate::test_helpers::load_short_input_for_day($day);
                let solver = crate::build_solver($day, input);
                let solution = solver.[<part $part>]();
                assert_eq!(solution, $expected_val);
            }

        }
    };
}

/// Generate tests for a given day
///
/// Most AdventOfCode puzzles include a short sample input, and a solution for each part of the
/// puzzle given that sample input. This macro is intended to generate tests with that input and
/// those sample solutions as they become available.
///
/// The tests automatically load the correct "short" input and build a `PuzzleSolver` for the given
/// day, then compare the solver's output to the given expected values. It can be called three ways:
/// * `test_short_input_for_day!(1)`: Generates two tests that are ignored
/// * `test_short_input_for_day!(1, 2400)`: Generates one test against day 1 part 1, expecting 100 as
///   the solution. Another test for part 2 is generated but ignored
/// * `test_short_input_for_day!(1, 2400, 3000)`: Generates two tests against both parts for day 1,
///   expecting 100 as the solution for part 1 and 200 as the solution for part 2
macro_rules! test_short_input_for_day {
    ($day:literal) => {
        paste::paste! {
            #[cfg(test)]
            mod [<_day $day _test>] {
                crate::test_helpers::_test_ignored!($day, 1);
                crate::test_helpers::_test_ignored!($day, 2);
            }
        }
    };
    ($day:literal, $part1_expected_val:expr) => {
        paste::paste! {
            #[cfg(test)]
            mod [<_day $day _test>] {
                use crate::PuzzleSolver;

                crate::test_helpers::_test!($day, 1, $part1_expected_val);
                crate::test_helpers::_test_ignored!($day, 2);
            }
        }
    };
    ($day:literal, $part1_expected_val:expr, $part2_expected_val:expr) => {
        paste::paste! {
            #[cfg(test)]
            mod [<_day $day _test>] {
                use crate::PuzzleSolver;

                crate::test_helpers::_test!($day, 1, $part1_expected_val);
                crate::test_helpers::_test!($day, 2, $part2_expected_val);
            }
        }
    };
}

pub(crate) use _test;
pub(crate) use _test_ignored;
pub(crate) use test_short_input_for_day;
