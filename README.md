# Advent of Code Framework for Rust

This repository contains a framework for writing solutions to the [Advent of Code][aoc] in [Rust](https://rust-lang.org). It is designed to make it easy to incrementally solve the puzzles, with the ability to automatically execute a given day's solution and generate tests against the sample inputs.

## Usage

Start by [forking the repository][repo] and branching off of the `framework` branch. You can create commits on your own branch (for example, `aoc-2022` for your 2022 solutions). Any updates to the framework can be integrated by merging `framework` into your branch  or rebasing your branch onto `framework`.

See the initial `day01.rs` for an example for how to set up a solution.

To implement a solution to a day's puzzle:
* Create a file called `src/solutions/dayXX.rs`, where `XX` is the day you're solving for (for example, `src/solutions/day10.rs`)
  * For single-digit days, prefix with a '0' for sorting
* Add the file as a module in `src/solutions/mod.rs`
* Add a line to the match statement in the `build_solver` function in `src/lib.rs`
* Add the full and short inputs to `inputs/dayXX{,-short}.txt`
* Create a struct for the day's puzzle that holds whatever data structure it needs and implements the `PuzzleSolution` trait
* Add these lines to the bottom of the file (replace 'XX' with the day, see macro documentation for details)

```rust
#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(XX);
```

The macro will generate a test for each part of that day's puzzle, testing your solution against the short input and its answer. You can run the tests with `cargo test`.

Run your solution against the full input for the day with `cargo run <DAY>`. Run `cargo run -- --help` for more options.


[aoc]: https://adventofcode.com/
[repo]: https://github.com/avandesa/adventofcode-rust-framework