use crate::{regex, PuzzleSolver};
use std::collections::VecDeque;

pub struct Day05 {
    actions: Vec<Action>,
    starting_stacks: Stacks,
}

impl PuzzleSolver for Day05 {
    fn with_input(input: String) -> Self {
        let num_stacks = (input.lines().next().unwrap().len() + 1) / 4;
        let mut starting_stacks = Stacks::new(num_stacks);

        let (stack_lines, action_lines) = input.split_once("\n\n").unwrap();
        let num_stack_lines = stack_lines.lines().count() - 1;

        // Matches "   " (three spaces) or "[<char>]", optionally followed by a space.
        // Extracts <char> as the name of the crate
        let box_or_empty_re = regex!(r#"(?:(?: {3}|\[(?P<crate>[A-Z])\])[ ]?)"#);

        for line in stack_lines.lines().take(num_stack_lines) {
            for (stack_num, space) in box_or_empty_re
                .captures_iter(line)
                .map(|cap| {
                    cap.name("crate")
                        .map(|m| m.as_str().chars().next().unwrap())
                })
                .enumerate()
                .filter_map(|(stack_num, space)| space.map(|space| (stack_num, space)))
            {
                starting_stacks.put_bottom(stack_num, space)
            }
        }

        let actions = action_lines
            .lines()
            .map(Action::from_line)
            .collect::<Vec<_>>();

        println!("Before:");
        println!("{starting_stacks}");

        Self {
            actions,
            starting_stacks,
        }
    }

    fn part1(&self) -> String {
        let mut stacks = self.starting_stacks.clone();

        for action in &self.actions {
            stacks.apply_action_single_crate(*action);
        }

        println!("{stacks}");

        stacks
            .0
            .iter()
            .map(|stack| stack.0.front().unwrap())
            .collect()
    }

    fn part2(&self) -> String {
        let mut stacks = self.starting_stacks.clone();

        for action in &self.actions {
            stacks.apply_action_multiple_crates(*action);
        }

        println!("{stacks}");

        stacks
            .0
            .iter()
            .map(|stack| stack.0.front().unwrap())
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn new(num_stacks: usize) -> Self {
        let stacks = std::iter::repeat(Stack::new())
            .take(num_stacks)
            .collect::<Vec<_>>();
        Self(stacks)
    }

    fn put_bottom(&mut self, stack: usize, c: char) {
        self.0[stack].put_bottom(c);
    }

    fn apply_action_single_crate(&mut self, action: Action) {
        for _ in 0..action.count {
            let popped = self.0[action.from - 1].pop_top();
            self.0[action.to - 1].push_top(popped);
        }
    }

    fn apply_action_multiple_crates(&mut self, action: Action) {
        let popped = self.0[action.from - 1].pop_top_n(action.count);
        self.0[action.to - 1].push_top_n(popped);
    }
}

#[derive(Debug, Clone)]
struct Stack(VecDeque<char>);

impl Stack {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push_top(&mut self, c: char) {
        self.0.push_front(c);
    }

    fn push_top_n(&mut self, crates: Vec<char>) {
        for c in crates.into_iter().rev() {
            self.0.push_front(c);
        }
    }

    fn put_bottom(&mut self, c: char) {
        self.0.push_back(c);
    }

    fn pop_top(&mut self) -> char {
        self.0.pop_front().expect("popped empty stack")
    }

    fn pop_top_n(&mut self, n: usize) -> Vec<char> {
        let mut rtn = Vec::with_capacity(n);
        for _ in 0..n {
            let popped = self.0.pop_front().expect("popped empty stack");
            rtn.push(popped);
        }
        rtn
    }
}

#[derive(Debug, Copy, Clone)]
struct Action {
    count: usize,
    from: usize,
    to: usize,
}

impl Action {
    fn from_line(line: &str) -> Self {
        let re = regex!(r#"^move (?P<stack>\d+) from (?P<from>\d+) to (?P<to>\d+)$"#);
        let captures = re.captures(line).expect("regex didn't match");
        Self {
            count: captures.name("stack").unwrap().as_str().parse().unwrap(),
            from: captures.name("from").unwrap().as_str().parse().unwrap(),
            to: captures.name("to").unwrap().as_str().parse().unwrap(),
        }
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<bottom>  ")?;
        for crate_name in &self.0 {
            write!(f, "{crate_name}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (stack_num, stack) in self.0.iter().enumerate() {
            writeln!(f, "{:2}:\t{}", stack_num, stack)?;
        }
        Ok(())
    }
}

#[cfg(test)]
crate::test_helpers::test_short_input_for_day!(5, "CMZ", "MCD");
