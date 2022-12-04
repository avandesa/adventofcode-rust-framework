use aoc_2022::build_solver;

use std::path::PathBuf;
use std::time::Instant;

use clap::ArgGroup;
use color_eyre::eyre::WrapErr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Args = clap::Parser::parse();
    let input_path = args.input_path();
    println!("Using input file: {input_path:#?}");

    let input = std::fs::read_to_string(&input_path)
        .wrap_err_with(|| format!("Failed to open input file: {:#?}", input_path))?;
    let solver = build_solver(args.day, input);

    println!("----- Solving part 1 -----");
    let part1_start = Instant::now();
    let part1 = solver.part1();
    let part1_duration = part1_start.elapsed();
    println!("--------------------------");
    println!("Part 1 solution: {}", part1);
    println!("Took {:#?}\n", part1_duration);

    println!("----- Solving part 2 -----");
    let part2_start = Instant::now();
    let part2 = solver.part2();
    let part2_duration = part2_start.elapsed();
    println!("--------------------------");
    println!("Part 2 solution: {}", part2);
    println!("Took {:#?}\n", part2_duration);

    Ok(())
}

#[derive(Debug, clap::Parser)]
#[command(
    group(
        ArgGroup::new("input")
            .required(false)
            .args(["short_input", "input_path"])
    )
)]
struct Args {
    /// Which day to solve (1-25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Whether or not to use the 'short' sample input for the given day
    #[arg(long, short)]
    short_input: bool,

    /// Override the input path
    #[arg(long)]
    input_path: Option<PathBuf>,
}

impl Args {
    fn input_path(&self) -> PathBuf {
        if let Some(input_path) = &self.input_path {
            input_path.clone()
        } else if self.short_input {
            PathBuf::from(format!("./inputs/day{:02}-short.txt", self.day))
        } else {
            PathBuf::from(format!("./inputs/day{:02}.txt", self.day))
        }
    }
}
