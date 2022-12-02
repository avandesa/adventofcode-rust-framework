use crate::PuzzleSolver;

pub struct Day02 {
    rounds: Vec<Round>,
}

impl PuzzleSolver for Day02 {
    fn with_input(input: String) -> Self {
        let rounds = input.lines().map(Round::from_line).collect();
        Self { rounds }
    }

    fn part1(&self) -> String {
        self.rounds
            .iter()
            .map(|round| round.score_part1() as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.rounds
            .iter()
            .map(|round| round.score_part2() as u32)
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug, Clone, Copy)]
enum MatchResult {
    Loss,
    Tie,
    Win,
}

impl MatchResult {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Tie,
            'Z' => Self::Win,
            _ => panic!("Invalid character: {c}"),
        }
    }

    fn points(&self) -> u8 {
        match self {
            Self::Loss => 0,
            Self::Tie => 3,
            Self::Win => 6,
        }
    }

    fn choice_for_outcome_against(&self, opponent_choice: Choice) -> Choice {
        match (self, opponent_choice) {
            (Self::Loss, Choice::Rock) => Choice::Scissors,
            (Self::Loss, Choice::Paper) => Choice::Rock,
            (Self::Loss, Choice::Scissors) => Choice::Paper,

            (Self::Win, Choice::Rock) => Choice::Paper,
            (Self::Win, Choice::Paper) => Choice::Scissors,
            (Self::Win, Choice::Scissors) => Choice::Rock,

            (Self::Tie, op) => op,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid character: {c}"),
        }
    }

    fn points(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn fight(&self, other: Self) -> MatchResult {
        match (self, other) {
            (Self::Rock, Self::Paper) => MatchResult::Loss,
            (Self::Rock, Self::Scissors) => MatchResult::Win,

            (Self::Paper, Self::Rock) => MatchResult::Win,
            (Self::Paper, Self::Scissors) => MatchResult::Loss,

            (Self::Scissors, Self::Rock) => MatchResult::Loss,
            (Self::Scissors, Self::Paper) => MatchResult::Win,

            _ => MatchResult::Tie,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent_choice: Choice,
    my_choice: Choice,
    desired_outcome: MatchResult,
}

impl Round {
    fn from_line(line: &str) -> Self {
        Self {
            opponent_choice: Choice::from_char(line.chars().nth(0).unwrap()),
            my_choice: Choice::from_char(line.chars().nth(2).unwrap()),
            desired_outcome: MatchResult::from_char(line.chars().nth(2).unwrap()),
        }
    }

    fn score_part1(&self) -> u8 {
        let choice_points = self.my_choice.points();
        let match_points = self.my_choice.fight(self.opponent_choice).points();
        choice_points + match_points
    }

    fn score_part2(&self) -> u8 {
        let match_points = self.desired_outcome.points();
        let required_choice = self
            .desired_outcome
            .choice_for_outcome_against(self.opponent_choice);
        let choice_points = required_choice.points();
        choice_points + match_points
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(2, "15", "12");
